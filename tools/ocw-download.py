#!/usr/bin/env python3
"""Download a course of MIT OpenCourseWare for `pdf2md-cli --course`.

    tools/ocw-download.py https://ocw.mit.edu/courses/<course>/ "<Course> Downloads"

Saves what each page of the course's menu holds as <page>.html (links made
whole addresses) and the PDFs each page lists into a folder named after the
page ("Lecture Notes", "Assignments"), and what is said in its videos, from
the caption files of the course's download package. Files that are there
are kept.
Needs curl. Then:

    pdf2md-cli --course "<Course> Downloads" "<Course>"
"""
import os, re, subprocess, sys, time

HOST = "https://ocw.mit.edu"


def fetch(url, to=None):
    cmd = ["curl", "-sL", "-A", "Mozilla/5.0"]
    if to:
        subprocess.run(cmd + ["-o", to, url], check=False)
        return None
    return subprocess.run(cmd + [url], capture_output=True, text=True).stdout


def title_of(stem):
    return " ".join(w.capitalize() for w in re.split(r"[-_ ]+", stem) if w)


def captions(url, course, folder, index):
    """What is said in the course's videos: the caption files of its
    download package, one for each video, named after the video's page so
    that the list of videos names them, and headed by the video's address."""
    import io, json, zipfile
    m = re.search(r'href="(%s/download/?)"' % re.escape(course), index)
    page = fetch(HOST + m.group(1)) if m else ""
    z = re.search(r'href="([^"]+\.zip)"', page)
    if not z:
        return 0
    data = subprocess.run(["curl", "-sL", "-A", "Mozilla/5.0", z.group(1) if z.group(1).startswith("http") else HOST + z.group(1)], capture_output=True).stdout
    try:
        package = zipfile.ZipFile(io.BytesIO(data))
    except zipfile.BadZipFile:
        return 0
    names = package.namelist()
    n = 0
    for name in sorted(names):
        if not re.fullmatch(r"resources/[^/]+/data\.json", name):
            continue
        d = json.loads(package.read(name))
        files = d.get("video_files") or {}
        vtt = files.get("video_captions_file") or ""
        if not vtt:
            continue
        inside = "static_resources/" + os.path.basename(vtt)
        if inside not in names:
            continue
        kinds = d.get("learning_resource_types") or ["Videos"]
        sub = os.path.join(folder, kinds[0])
        os.makedirs(sub, exist_ok=True)
        text = package.read(inside).decode("utf-8", "replace")
        head = ["WEBVTT", ""]
        video = (d.get("video_metadata") or {}).get("youtube_id")
        if video:
            head += ["NOTE video https://www.youtube.com/watch?v=" + video, ""]
        about = re.sub(r"\s+", " ", re.sub(r"<[^>]+>", "", d.get("content") or "")).strip()
        if about:
            head += ["NOTE about " + about, ""]
        text = re.sub(r"^\ufeff?WEBVTT[^\n]*\n", "", text, count=1)
        with open(os.path.join(sub, name.split("/")[1] + ".webvtt"), "w") as f:
            f.write("\n".join(head) + text)
        n += 1
    if n:
        print("captions of %d videos" % n)
    return n


def main():
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    url, folder = sys.argv[1].rstrip("/"), sys.argv[2]
    course = "/courses/" + url.rsplit("/courses/", 1)[-1]
    os.makedirs(folder, exist_ok=True)
    index = fetch(url + "/")
    # the pages of the course's menu, in its order
    pages = []
    for kind, name in re.findall(r'href="%s/(pages|lists|video_galleries)/([^"/]+)/?"' % re.escape(course), index):
        if (kind, name) not in pages:
            pages.append((kind, name))
    got, missing = 0, []
    for kind, name in pages:
        html = fetch("%s/%s/%s/" % (url, kind, name))
        m = re.search(r'<main id="course-content-section">(.*?)</main>', html, re.S)
        if not m:
            continue
        body = m.group(1)
        if not body.strip():
            # (a page that is a list of files or of videos has them after it)
            rest = html[m.end():]
            last = max(rest.rfind("resource-item"), rest.rfind("video-gallery-card"), rest.rfind("%s/resources/" % course))
            ends = [k for k in (rest.find("<footer", last), rest.find("<script", last), rest.find("</a>", last) + 4) if k >= max(last, 4)]
            body = rest[: min(ends)] if ends else rest
        # the PDFs the page lists: by the page of each, or straight
        files = []
        key = lambda t: re.sub(r"[^a-z0-9]", "", t.lower())
        for pdf in re.findall(r'href="(%s/[^"/]+\.pdf)"' % re.escape(course), body):
            if pdf not in [f for _, f in files]:
                files.append((pdf, pdf))
        have = [key(os.path.basename(f)[:-4]) for _, f in files]
        for res in re.findall(r'href="(%s/resources/[^"]+)"' % re.escape(course), body):
            slug = key(res.rstrip("/").rsplit("/", 1)[-1])
            if res in [r for r, _ in files] or res.rstrip("/").endswith(("_mp4", "_mp3")) or slug in have or slug[:-3] in have:
                continue
            page = fetch(HOST + res)
            pdf = re.search(r'href="(%s/[^"/]+\.pdf)"' % re.escape(course), page)
            if pdf:
                files.append((res, pdf.group(1)))
            time.sleep(0.2)
        body = re.sub(r'(href|src)="/(?!/)', r'\1="%s/' % HOST, body)
        with open(os.path.join(folder, name + ".html"), "w") as f:
            f.write('<!DOCTYPE html>\n<html lang="en">\n  <body>\n    <main id="course-content-section">%s</main>\n  </body>\n</html>\n' % body)
        if not files:
            continue
        sub = os.path.join(folder, title_of(name))
        os.makedirs(sub, exist_ok=True)
        for res, pdf in files:
            # (the site puts a number of 32 letters before the name)
            out = os.path.join(sub, re.sub(r"^[0-9a-f]{32}_", "", os.path.basename(pdf)))
            if not os.path.exists(out):
                fetch(HOST + pdf, out)
                time.sleep(0.3)
            if os.path.exists(out) and open(out, "rb").read(5) == b"%PDF-":
                got += 1
            else:
                missing.append(res)
        print("%s: %d PDFs" % (title_of(name), len(files)))
    got += captions(url, course, folder, index)
    print("%d files in %s" % (got, folder))
    for m in missing:
        print("not a PDF:", m)


if __name__ == "__main__":
    main()
