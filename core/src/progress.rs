//! How far a conversion is: the steps it goes through, how much of each is
//! done, and an estimate of the time left.
//!
//! The converter calls `report` as it goes; whoever wants to show progress
//! (the app) hands `watch` a function to receive the reports, and can feed
//! them to a `Tracker`, which keeps the times and estimates what is left.

use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::time::Instant;

/// The steps of a conversion that take time, in the order they run.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Step {
    /// `pdftohtml`: the text with its positions and fonts (counted in pages)
    Text,
    /// `pdftocairo -svg`: drawn bars, brackets, pictures (pages)
    Drawings,
    /// the pages of a scanned book read by the page reader (pages)
    Scan,
    /// the marks of footnotes looked for on the pictures of pages (pages)
    Marks,
    /// images looked at: a drawing, or something to read? (images)
    Sorting,
    /// images read as plain text, to find program code among them (images)
    Words,
    /// the math recognizer's model, loaded
    Model,
    /// equation images read by the math recognizer (images)
    Formulas,
    /// the ones not read plausibly, read again in other ways (images)
    Again,
    /// equations of several lines read as one line, taken apart (images)
    Lines,
    /// words under an equation, taken out of its reading (images)
    Labels,
    /// all read a second time, to list the doubtful ones (images)
    Check,
    /// chapters, notes, links
    Book,
    /// pictures cut out of the PDF (pictures)
    Pictures,
    /// the Markdown files
    Files,
}

/// The steps of the math recognizer: they show only once it runs.
const RECOGNIZER: [Step; 8] =
    [Step::Sorting, Step::Words, Step::Model, Step::Formulas, Step::Again, Step::Lines, Step::Labels, Step::Check];

impl Step {
    pub const ALL: [Step; 15] = [
        Step::Text,
        Step::Drawings,
        Step::Scan,
        Step::Marks,
        Step::Sorting,
        Step::Words,
        Step::Model,
        Step::Formulas,
        Step::Again,
        Step::Lines,
        Step::Labels,
        Step::Check,
        Step::Book,
        Step::Pictures,
        Step::Files,
    ];

    /// What the step does, for showing.
    pub fn label(self) -> &'static str {
        match self {
            Step::Text => "Reading the text",
            Step::Drawings => "Reading drawings and pictures",
            Step::Scan => "Reading the scanned pages",
            Step::Marks => "Looking for footnote marks",
            Step::Sorting => "Telling drawings from equations",
            Step::Words => "Looking for program code",
            Step::Model => "Loading the equation reader",
            Step::Formulas => "Reading equation images",
            Step::Again => "Reading doubtful equations again",
            Step::Lines => "Looking for lines run together",
            Step::Labels => "Looking for words under equations",
            Step::Check => "Reading equations a second time",
            Step::Book => "Building chapters, notes and links",
            Step::Pictures => "Cutting out the pictures",
            Step::Files => "Writing the files",
        }
    }

    /// What its amounts count.
    pub fn unit(self) -> &'static str {
        match self {
            Step::Text | Step::Drawings | Step::Scan | Step::Marks => "pages",
            Step::Sorting | Step::Words => "images",
            Step::Formulas | Step::Again | Step::Lines | Step::Labels | Step::Check => "equations",
            Step::Pictures => "pictures",
            Step::Model | Step::Book | Step::Files => "",
        }
    }

    /// Seconds a unit of the step takes before any of it is measured, and
    /// seconds the step takes besides (starting a program, loading the
    /// recognizer's model). Several times what was measured on one machine
    /// with three books, on purpose: a conversion that is over sooner than
    /// it said worries nobody, one that takes longer does.
    fn guess(self) -> (f32, f32) {
        match self {
            Step::Text => (0.01, 1.0),
            Step::Drawings => (0.05, 2.0),
            // (with the page reader's models loaded first)
            Step::Scan => (6.0, 60.0),
            Step::Marks => (3.0, 2.0),
            Step::Sorting => (0.05, 2.0),
            Step::Words => (0.1, 2.0),
            Step::Model => (0.0, 40.0),
            Step::Formulas => (0.2, 0.0),
            Step::Again => (4.0, 0.0),
            Step::Lines => (0.1, 0.0),
            Step::Labels => (0.05, 0.0),
            Step::Check => (0.3, 0.0),
            Step::Book => (0.0, 2.0),
            Step::Pictures => (0.15, 0.0),
            Step::Files => (0.0, 1.0),
        }
    }
}

type Sink = Box<dyn Fn(Step, usize, usize) + Send>;

static SINK: Mutex<Option<Sink>> = Mutex::new(None);

/// Receive the reports of conversions from now on: (step, done, total).
/// A step begins with done = 0 and ends with done = total; 0 of 0 says the
/// step is left out this time.
pub fn watch(sink: impl Fn(Step, usize, usize) + Send + 'static) {
    *SINK.lock().unwrap_or_else(|e| e.into_inner()) = Some(Box::new(sink));
}

/// Stop receiving reports.
pub fn unwatch() {
    *SINK.lock().unwrap_or_else(|e| e.into_inner()) = None;
}

/// This conversion does without the step.
pub fn skip(step: Step) {
    report(step, 0, 0);
}

/// `done` of `total` of a step are done.
pub fn report(step: Step, done: usize, total: usize) {
    if let Some(sink) = SINK.lock().unwrap_or_else(|e| e.into_inner()).as_ref() {
        sink(step, done, total);
    }
}

/// How many pages a PDF has (`pdfinfo`); 0 if that cannot be told.
pub fn page_count(pdf: &std::path::Path) -> usize {
    Command::new("pdfinfo")
        .arg(pdf)
        .output()
        .ok()
        .and_then(|o| String::from_utf8_lossy(&o.stdout).lines().find_map(|l| l.strip_prefix("Pages:").and_then(|n| n.trim().parse().ok())))
        .unwrap_or(0)
}

/// Run a program that prints one `mark` per page, and report the pages as
/// they come. Gives back whether it succeeded, what it printed, and its
/// error messages.
pub fn run_counting(mut cmd: Command, mark: &[u8], step: Step, total: usize) -> std::io::Result<(bool, Vec<u8>, Vec<u8>)> {
    report(step, 0, total);
    let mut child = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).stdin(Stdio::null()).spawn()?;
    // (read the error messages meanwhile, or a full pipe would stop the program)
    let mut errors = child.stderr.take();
    let errors = std::thread::spawn(move || {
        let mut e = Vec::new();
        if let Some(pipe) = errors.as_mut() {
            let _ = pipe.read_to_end(&mut e);
        }
        e
    });
    let mut out = Vec::new();
    let mut pages = 0;
    if let Some(mut pipe) = child.stdout.take() {
        let mut buf = vec![0u8; 1 << 16];
        loop {
            let n = pipe.read(&mut buf)?;
            if n == 0 {
                break;
            }
            // marks in the new piece, also one begun in the piece before
            let from = out.len().saturating_sub(mark.len() - 1);
            out.extend_from_slice(&buf[..n]);
            let new = out[from..].windows(mark.len()).filter(|w| *w == mark).count();
            if new > 0 {
                pages += new;
                // (the page whose mark came last is still being printed)
                report(step, (pages - 1).min(total.saturating_sub(1)), total);
            }
        }
    }
    let ok = child.wait()?.success();
    Ok((ok, out, errors.join().unwrap_or_default()))
}

/// One step as a `Tracker` sees it.
#[derive(Clone, Debug, PartialEq)]
pub struct StepView {
    pub step: Step,
    pub state: State,
    /// how much of it is done, 0 to 1
    pub fraction: f32,
    pub done: usize,
    pub total: usize,
    /// seconds: how long it took (done), or is estimated to take still
    pub seconds: f32,
    /// seconds it has been running (0 unless it runs)
    pub spent: f32,
    /// seconds since its count last grew (0 unless it runs)
    pub quiet: f32,
    /// seconds it usually takes for the count to grow, as measured (0 if
    /// not measured yet): a step that counts in lots is quiet in between
    pub pace: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    Waiting,
    Running,
    Done,
}

#[derive(Clone, Copy)]
struct Timing {
    step: Step,
    done: usize,
    total: usize,
    started: Option<Instant>,
    /// when the first unit was done: the time before is not per unit
    first: Option<(Instant, usize)>,
    /// when `done` last grew, and by how much
    latest: Option<Instant>,
    stride: usize,
    finished: Option<Instant>,
    /// left out of this conversion
    skipped: bool,
}

/// A measured speed counts for this much more than it says: later units
/// are often bigger than the first ones.
const CAUTION: f32 = 1.25;

/// Keeps the reports of one conversion with their times, and estimates
/// what is left.
pub struct Tracker {
    began: Instant,
    pages: usize,
    steps: Vec<Timing>,
}

fn secs(a: Instant, b: Instant) -> f32 {
    b.saturating_duration_since(a).as_secs_f32()
}

impl Tracker {
    pub fn new(now: Instant) -> Self {
        let steps = Step::ALL
            .iter()
            // (only a scanned book is read by the page reader)
            .map(|&step| Timing { step, done: 0, total: 0, started: None, first: None, latest: None, stride: 1, finished: None, skipped: matches!(step, Step::Scan | Step::Marks) })
            .collect();
        Tracker { began: now, pages: 0, steps }
    }

    /// Take a report in.
    pub fn update(&mut self, step: Step, done: usize, total: usize, now: Instant) {
        if step == Step::Text && total > 0 {
            self.pages = total;
        }
        let at = Step::ALL.iter().position(|s| *s == step).unwrap_or(0);
        if total == 0 {
            // nothing to do in it
            if self.steps[at].started.is_none() {
                self.steps[at].skipped = true;
            }
            return;
        }
        // a step that reports is running: the ones before it are over, or
        // were left out
        for t in &mut self.steps[..at] {
            match t.started {
                Some(_) if t.finished.is_none() => {
                    t.finished = Some(now);
                    t.done = t.total;
                }
                None => t.skipped = true,
                _ => {}
            }
        }
        let t = &mut self.steps[at];
        t.skipped = false;
        t.started.get_or_insert(now);
        let done = done.min(total);
        if done > 0 && t.first.is_none() {
            t.first = Some((now, done));
        }
        if done > t.done || t.latest.is_none() {
            if done > t.done {
                t.stride = done - t.done;
            }
            t.latest = Some(now);
        }
        (t.done, t.total) = (done, total);
        t.finished = (done >= total).then_some(now);
    }

    fn timing(&self, step: Step) -> Option<&Timing> {
        self.steps.iter().find(|t| t.step == step)
    }

    /// Is the math recognizer at work in this conversion?
    fn recognizing(&self) -> bool {
        self.steps.iter().any(|t| RECOGNIZER.contains(&t.step) && t.started.is_some())
    }

    /// Seconds a unit took, of a step that has done some.
    fn measured(t: &Timing) -> Option<f32> {
        let (started, (first, n), latest) = (t.started?, t.first?, t.latest?);
        Some(if t.done > n {
            secs(first, latest) / (t.done - n) as f32
        } else {
            // one report only: with all the time before it
            secs(started, first) / n as f32
        })
    }

    /// Seconds a step that has not begun is estimated to take.
    fn ahead(&self, step: Step) -> f32 {
        let (per_unit, besides) = step.guess();
        let pages = self.pages as f32;
        // how many images the recognizer was given, as far as known
        let images = [Step::Formulas, Step::Sorting]
            .iter()
            .filter_map(|s| self.timing(*s))
            .find(|t| t.started.is_some())
            .map_or(0.0, |t| t.total as f32);
        // the speed of the first reading says more about the later ones
        // than a guess does (the second reading is of enlarged images)
        let reading = self.timing(Step::Formulas).and_then(Self::measured).map(|r| r * CAUTION);
        let units = match step {
            Step::Text | Step::Drawings | Step::Scan => pages,
            // (seldom more than one picture to three pages)
            Step::Pictures => 0.3 * pages,
            // (how many must be read again is not known before)
            Step::Again => 0.05 * images,
            Step::Sorting | Step::Words | Step::Formulas | Step::Lines | Step::Labels | Step::Check => images,
            Step::Marks | Step::Model | Step::Book | Step::Files => 0.0,
        };
        let rate = match (step, reading) {
            (Step::Lines, Some(r)) => per_unit.max(0.5 * r),
            (Step::Check, Some(r)) => per_unit.max(1.5 * r),
            _ => per_unit,
        };
        besides + units * rate
    }

    /// Seconds a step is estimated to take still.
    fn left(&self, t: &Timing, now: Instant) -> f32 {
        let Some(started) = t.started else {
            // not begun: the recognizer runs only for books with new
            // equation images, which is not known before it starts
            let never = t.skipped || (RECOGNIZER.contains(&t.step) && !self.recognizing());
            return if never { 0.0 } else { self.ahead(t.step) };
        };
        if t.finished.is_some() {
            return 0.0;
        }
        let (per_unit, besides) = t.step.guess();
        let rest = t.total.saturating_sub(t.done) as f32;
        let quiet = secs(t.latest.unwrap_or(started), now);
        match (Self::measured(t), t.first) {
            (Some(rate), Some((first, n))) => {
                // at the speed measured (time since the latest report
                // counts against the units under way) -- but when they are
                // late, at the speed they would have if they came now
                let on_time = rest * rate * CAUTION - quiet;
                let from = if t.done > n { first } else { started };
                let late = rest * secs(from, now) / (t.done.max(1) - if t.done > n { n } else { 0 } + t.stride) as f32;
                // and something quiet for long may be as long again
                let stuck = 0.5 * (quiet - 2.0 * rate * t.stride as f32);
                on_time.max(late).max(stuck).max(1.0)
            }
            // nothing measured yet: the guess, less the time spent (some
            // programs print all their pages at the end); past the guess,
            // half as long again as it has taken so far
            _ => {
                let spent = secs(started, now);
                (besides + rest * per_unit - spent).max(0.5 * spent).max(1.0)
            }
        }
    }

    /// Seconds the conversion is estimated to take still.
    pub fn remaining(&self, now: Instant) -> f32 {
        self.steps.iter().map(|t| self.left(t, now)).sum()
    }

    /// Seconds since the conversion began.
    pub fn spent(&self, now: Instant) -> f32 {
        secs(self.began, now)
    }

    /// How much of the conversion is done, 0 to 1, by time.
    pub fn fraction(&self, now: Instant) -> f32 {
        let spent = self.spent(now);
        let all = spent + self.remaining(now);
        if all > 0.0 {
            (spent / all).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    /// The steps that have begun and the ones to come.
    pub fn steps(&self, now: Instant) -> Vec<StepView> {
        // (the recognizer's steps only once it runs, and no step left out)
        let recognizing = self.recognizing();
        self.steps
            .iter()
            .filter(|t| !t.skipped && (t.started.is_some() || recognizing || !RECOGNIZER.contains(&t.step)))
            .map(|t| {
                let left = self.left(t, now);
                let (state, fraction, seconds, spent, quiet) = match (t.started, t.finished) {
                    (Some(s), Some(f)) => (State::Done, 1.0, secs(s, f), 0.0, 0.0),
                    (Some(s), None) => {
                        let spent = secs(s, now);
                        (State::Running, spent / (spent + left), left, spent, secs(t.latest.unwrap_or(s), now))
                    }
                    (None, _) => (State::Waiting, 0.0, left, 0.0, 0.0),
                };
                let pace = if state == State::Running { Self::measured(t).map_or(0.0, |rate| rate * t.stride as f32) } else { 0.0 };
                StepView { step: t.step, state, fraction, done: t.done, total: t.total, seconds, spent, quiet, pace }
            })
            .collect()
    }
}

/// "2 min 5 s", "45 s": time gone by, in whole seconds (it is shown
/// ticking, to tell that the conversion is alive).
pub fn elapsed(seconds: f32) -> String {
    let s = seconds.max(0.0) as u64;
    match s / 60 {
        0 => format!("{s} s"),
        m => format!("{m} min {} s", s % 60),
    }
}

/// "3 min 20 s", "45 s", "2 s": a time for showing; estimates rounded so
/// they do not flicker.
pub fn duration(seconds: f32, estimate: bool) -> String {
    // (a short time that was measured: "0.3 s")
    if !estimate && seconds < 9.95 {
        return format!("{:.1} s", seconds.max(0.0));
    }
    let mut s = seconds.max(0.0).round() as u64;
    if estimate {
        s = match s {
            0..=10 => s,
            11..=60 => s.div_ceil(5) * 5,
            _ => s.div_ceil(10) * 10,
        };
    }
    match (s / 60, s % 60) {
        (0, s) => format!("{s} s"),
        (m, 0) => format!("{m} min"),
        (m, s) => format!("{m} min {s} s"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn the_time_left_follows_the_speed_measured() {
        let t0 = Instant::now();
        let at = |s: f32| t0 + Duration::from_secs_f32(s);
        let mut t = Tracker::new(t0);
        t.update(Step::Text, 0, 100, at(0.0));
        // 10 equations a second: 890 left take 89 s, said with caution
        t.update(Step::Sorting, 0, 1000, at(1.0));
        t.update(Step::Sorting, 1000, 1000, at(2.0));
        t.update(Step::Formulas, 0, 1000, at(6.0));
        t.update(Step::Formulas, 10, 1000, at(6.0));
        t.update(Step::Formulas, 110, 1000, at(16.0));
        let view = t.steps(at(16.0));
        let formulas = view.iter().find(|v| v.step == Step::Formulas).unwrap();
        assert_eq!(formulas.state, State::Running);
        assert!((formulas.seconds - 89.0 * CAUTION).abs() < 0.5, "{}", formulas.seconds);
        // the steps before it are over, or were left out (nothing was
        // read as text, no model was loaded); the second reading is to come
        assert_eq!(view[0].state, State::Done);
        assert!(view.iter().all(|v| v.step != Step::Words && v.step != Step::Model));
        assert_eq!(view.iter().find(|v| v.step == Step::Check).unwrap().state, State::Waiting);
        // still to come: all of them read a second time, no faster, and
        // looked at for lines run together
        assert!(t.remaining(at(16.0)) >= 89.0 + 100.0);
        assert!(t.fraction(at(16.0)) > 0.01 && t.fraction(at(16.0)) < 0.08);
        // done: nothing left of it
        t.update(Step::Formulas, 1000, 1000, at(105.0));
        assert_eq!(t.steps(at(105.0)).iter().find(|v| v.step == Step::Formulas).unwrap().state, State::Done);
    }

    #[test]
    fn a_step_that_is_late_says_so() {
        let t0 = Instant::now();
        let at = |s: f32| t0 + Duration::from_secs_f32(s);
        let left = |t: &Tracker, step: Step, s: f32| t.steps(at(s)).iter().find(|v| v.step == step).unwrap().seconds;
        let mut t = Tracker::new(t0);
        t.update(Step::Text, 100, 100, at(0.0));
        // nothing counted yet, far past the guess: the time left grows
        // with the time gone, it does not stay at a second
        t.update(Step::Sorting, 0, 500, at(0.0));
        let guess = left(&t, Step::Sorting, 0.0);
        assert!(guess >= 20.0, "{guess}");
        assert!(left(&t, Step::Sorting, 10.0) < guess);
        assert!((left(&t, Step::Sorting, 300.0) - 150.0).abs() < 1.0);
        assert!(left(&t, Step::Sorting, 600.0) > left(&t, Step::Sorting, 300.0));
        let running = t.steps(at(300.0)).into_iter().find(|v| v.step == Step::Sorting).unwrap();
        assert_eq!((running.spent.round(), running.quiet.round()), (300.0, 300.0));
        // 16 counted after 10 minutes: the others are taken to be as slow
        t.update(Step::Sorting, 16, 500, at(600.0));
        assert!(left(&t, Step::Sorting, 600.0) > 484.0 * 600.0 / 16.0);
        // steady counting, then silence: the estimate turns and grows
        let mut t = Tracker::new(t0);
        t.update(Step::Formulas, 0, 320, at(0.0));
        for n in 1..=10 {
            t.update(Step::Formulas, 16 * n, 320, at(n as f32));
        }
        let steady = left(&t, Step::Formulas, 10.0);
        assert!((steady - 10.0 * CAUTION).abs() < 0.5, "{steady}");
        assert!(left(&t, Step::Formulas, 100.0) > 3.0 * steady);
        assert!(left(&t, Step::Formulas, 200.0) > left(&t, Step::Formulas, 100.0));
        // a report that brings nothing new does not end the silence
        t.update(Step::Formulas, 160, 320, at(150.0));
        let quiet = t.steps(at(200.0)).into_iter().find(|v| v.step == Step::Formulas).unwrap();
        assert_eq!(quiet.quiet.round(), 190.0);
        // (16 came every second before)
        assert!((quiet.pace - 1.0).abs() < 0.01, "{}", quiet.pace);
    }

    #[test]
    fn steps_left_out_do_not_show() {
        let t0 = Instant::now();
        let mut t = Tracker::new(t0);
        t.update(Step::Text, 0, 100, t0);
        // no recognizer: none of its steps, and no time for them
        assert!(t.steps(t0).iter().all(|v| !RECOGNIZER.contains(&v.step)));
        let before = t.remaining(t0);
        // only checks of readings made before
        for step in [Step::Words, Step::Formulas, Step::Again] {
            t.update(step, 0, 0, t0);
        }
        t.update(Step::Sorting, 0, 50, t0);
        let shown: Vec<Step> = t.steps(t0).iter().map(|v| v.step).filter(|s| RECOGNIZER.contains(s)).collect();
        assert_eq!(shown, [Step::Sorting, Step::Model, Step::Lines, Step::Labels, Step::Check]);
        assert!(t.remaining(t0) > before);
        // no pictures to cut out
        t.update(Step::Pictures, 0, 0, t0);
        assert!(t.steps(t0).iter().all(|v| v.step != Step::Pictures));
    }

    #[test]
    fn times_are_written_plainly() {
        assert_eq!(duration(4.44, false), "4.4 s");
        assert_eq!(duration(4.4, true), "4 s");
        assert_eq!(duration(200.0, false), "3 min 20 s");
        assert_eq!(duration(47.0, true), "50 s");
        assert_eq!(duration(173.0, true), "3 min");
        assert_eq!(elapsed(59.9), "59 s");
        assert_eq!(elapsed(125.2), "2 min 5 s");
    }
}
