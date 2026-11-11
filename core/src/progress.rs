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
    /// equation images read by the math recognizer (images)
    Formulas,
    /// the ones not read plausibly, read again in other ways (images)
    Again,
    /// equations of several lines read as one line, taken apart (images)
    Lines,
    /// all read a second time, to list the doubtful ones (images)
    Check,
    /// chapters, notes, links
    Book,
    /// pictures cut out of the PDF (pictures)
    Pictures,
    /// the Markdown files
    Files,
}

impl Step {
    pub const ALL: [Step; 9] =
        [Step::Text, Step::Drawings, Step::Formulas, Step::Again, Step::Lines, Step::Check, Step::Book, Step::Pictures, Step::Files];

    /// What the step does, for showing.
    pub fn label(self) -> &'static str {
        match self {
            Step::Text => "Reading the text",
            Step::Drawings => "Reading drawings and pictures",
            Step::Formulas => "Reading equation images",
            Step::Again => "Reading doubtful equations again",
            Step::Lines => "Looking for lines run together",
            Step::Check => "Reading equations a second time",
            Step::Book => "Building chapters, notes and links",
            Step::Pictures => "Cutting out the pictures",
            Step::Files => "Writing the files",
        }
    }

    /// What its amounts count.
    pub fn unit(self) -> &'static str {
        match self {
            Step::Text | Step::Drawings => "pages",
            Step::Formulas | Step::Again | Step::Lines | Step::Check => "equations",
            Step::Pictures => "pictures",
            Step::Book | Step::Files => "",
        }
    }

    /// Seconds a unit of the step takes before any of it is measured, and
    /// seconds the step takes besides (starting a program, loading the
    /// recognizer's model). Rough: measured on one machine with two books.
    fn guess(self) -> (f32, f32) {
        match self {
            Step::Text => (0.002, 0.1),
            Step::Drawings => (0.006, 0.3),
            Step::Formulas => (0.04, 15.0),
            Step::Again => (1.0, 0.0),
            Step::Lines => (0.03, 0.0),
            Step::Check => (0.05, 0.0),
            Step::Book => (0.0, 0.4),
            Step::Pictures => (0.045, 0.0),
            Step::Files => (0.0, 0.1),
        }
    }
}

type Sink = Box<dyn Fn(Step, usize, usize) + Send>;

static SINK: Mutex<Option<Sink>> = Mutex::new(None);

/// Receive the reports of conversions from now on: (step, done, total).
/// A step begins with done = 0 and ends with done = total.
pub fn watch(sink: impl Fn(Step, usize, usize) + Send + 'static) {
    *SINK.lock().unwrap_or_else(|e| e.into_inner()) = Some(Box::new(sink));
}

/// Stop receiving reports.
pub fn unwatch() {
    *SINK.lock().unwrap_or_else(|e| e.into_inner()) = None;
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
    latest: Option<Instant>,
    finished: Option<Instant>,
}

/// Keeps the reports of one conversion with their times, and estimates
/// what is left.
pub struct Tracker {
    began: Instant,
    pages: usize,
    steps: Vec<Timing>,
}

impl Tracker {
    pub fn new(now: Instant) -> Self {
        let steps = Step::ALL.iter().map(|&step| Timing { step, done: 0, total: 0, started: None, first: None, latest: None, finished: None }).collect();
        Tracker { began: now, pages: 0, steps }
    }

    /// Take a report in.
    pub fn update(&mut self, step: Step, done: usize, total: usize, now: Instant) {
        if step == Step::Text && total > 0 {
            self.pages = total;
        }
        // a step that reports is running: the ones before it are over
        let at = Step::ALL.iter().position(|s| *s == step).unwrap_or(0);
        for t in &mut self.steps[..at] {
            if t.started.is_some() && t.finished.is_none() {
                t.finished = Some(now);
                t.done = t.total;
            }
        }
        let t = &mut self.steps[at];
        t.started.get_or_insert(now);
        (t.done, t.total) = (done.min(total), total);
        if done > 0 && t.first.is_none() {
            t.first = Some((now, done));
        }
        t.latest = Some(now);
        t.finished = (done >= total).then_some(now);
    }

    /// Seconds a step is estimated to take still.
    fn left(&self, t: &Timing, now: Instant) -> f32 {
        let (per_unit, besides) = t.step.guess();
        let secs = |a: Instant, b: Instant| b.saturating_duration_since(a).as_secs_f32();
        match (t.started, t.finished) {
            (_, Some(_)) => 0.0,
            // not begun: by the number of pages, if the step counts pages;
            // the recognizer runs only for books with new equation images,
            // which is not known before it starts
            (None, None) => match t.step {
                Step::Text | Step::Drawings => besides + per_unit * self.pages as f32,
                // (about one picture to eight pages)
                Step::Pictures => per_unit * 0.13 * self.pages as f32,
                // all equations are read a second time and looked at for
                // lines, about as fast as the first time; how many must be
                // read again is not known before
                Step::Lines | Step::Check => {
                    let formulas = self.steps.iter().find(|f| f.step == Step::Formulas && f.started.is_some());
                    formulas.map_or(0.0, |f| {
                        let rate = match (f.first, f.latest) {
                            (Some((first, n)), Some(latest)) if f.done > n => secs(first, latest) / (f.done - n) as f32,
                            _ => Step::Formulas.guess().0,
                        };
                        f.total as f32 * rate * if t.step == Step::Lines { 0.5 } else { 1.0 }
                    })
                }
                Step::Formulas | Step::Again => 0.0,
                Step::Book | Step::Files => besides,
            },
            (Some(started), None) => {
                let rest = t.total.saturating_sub(t.done) as f32;
                match t.first {
                    // at the speed measured since the first units were done
                    Some((first, n)) if t.done > n => {
                        let rate = secs(first, t.latest.unwrap_or(now)) / (t.done - n) as f32;
                        // (time since the latest report counts against the next unit)
                        (rest * rate - secs(t.latest.unwrap_or(now), now)).max(0.0)
                    }
                    // nothing measured yet: the guess, less the time spent
                    // (some programs print all their pages at the end)
                    _ => (besides + rest * per_unit - secs(started, now)).max(0.5),
                }
            }
        }
    }

    /// Seconds the conversion is estimated to take still.
    pub fn remaining(&self, now: Instant) -> f32 {
        self.steps.iter().map(|t| self.left(t, now)).sum()
    }

    /// Seconds since the conversion began.
    pub fn spent(&self, now: Instant) -> f32 {
        now.saturating_duration_since(self.began).as_secs_f32()
    }

    /// How much of the conversion is done, 0 to 1, by time.
    pub fn fraction(&self, now: Instant) -> f32 {
        let spent = now.saturating_duration_since(self.began).as_secs_f32();
        let all = spent + self.remaining(now);
        if all > 0.0 {
            (spent / all).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    /// The steps that have begun and the ones to come.
    pub fn steps(&self, now: Instant) -> Vec<StepView> {
        let secs = |a: Instant, b: Instant| b.saturating_duration_since(a).as_secs_f32();
        // (the recognizer's steps only once it runs)
        let recognizing = self.steps.iter().any(|t| t.step == Step::Formulas && t.started.is_some());
        self.steps
            .iter()
            .filter(|t| t.started.is_some() || recognizing || !matches!(t.step, Step::Formulas | Step::Again | Step::Lines | Step::Check))
            .map(|t| {
                let left = self.left(t, now);
                let (state, fraction, seconds) = match (t.started, t.finished) {
                    (Some(s), Some(f)) => (State::Done, 1.0, secs(s, f)),
                    (Some(s), None) => {
                        let spent = secs(s, now);
                        (State::Running, if spent + left > 0.0 { spent / (spent + left) } else { 0.0 }, left)
                    }
                    (None, _) => (State::Waiting, 0.0, left),
                };
                StepView { step: t.step, state, fraction, done: t.done, total: t.total, seconds }
            })
            .collect()
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
        // 5 s of loading, then 10 equations a second: 990 left take 99 s
        t.update(Step::Formulas, 0, 1000, at(1.0));
        t.update(Step::Formulas, 0, 1000, at(6.0));
        t.update(Step::Formulas, 10, 1000, at(6.0));
        t.update(Step::Formulas, 110, 1000, at(16.0));
        let view = t.steps(at(16.0));
        let formulas = view.iter().find(|v| v.step == Step::Formulas).unwrap();
        assert_eq!(formulas.state, State::Running);
        assert!((formulas.seconds - 89.0).abs() < 0.5, "{}", formulas.seconds);
        // the step before it is over; the second reading is still to come
        assert_eq!(view[0].state, State::Done);
        assert_eq!(view.iter().find(|v| v.step == Step::Check).unwrap().state, State::Waiting);
        // still to come: all of them read a second time, as fast (100 s),
        // and looked at for lines run together
        assert!(t.remaining(at(16.0)) >= 89.0 + 100.0);
        assert!(t.fraction(at(16.0)) > 0.04 && t.fraction(at(16.0)) < 0.08);
        // done: nothing left of it
        t.update(Step::Formulas, 1000, 1000, at(105.0));
        assert_eq!(t.steps(at(105.0)).iter().find(|v| v.step == Step::Formulas).unwrap().state, State::Done);
    }

    #[test]
    fn times_are_written_plainly() {
        assert_eq!(duration(4.44, false), "4.4 s");
        assert_eq!(duration(4.4, true), "4 s");
        assert_eq!(duration(200.0, false), "3 min 20 s");
        assert_eq!(duration(47.0, true), "50 s");
        assert_eq!(duration(173.0, true), "3 min");
    }
}
