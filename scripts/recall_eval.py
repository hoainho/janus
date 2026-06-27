#!/usr/bin/env python3
"""recall_eval.py — Evaluate ws-memories recall quality.

Answers: "How do we know recalled atoms are correct + fresh?"
Three objective measures, no daemon required:
  1. FRESHNESS  — age by `verified_at` (NOT file mtime, which lies after re-sync).
  2. CODE-DRIFT — for atoms citing `path:line`, is the cited file still on disk?
  3. PRECISION@k — seed golden set (query -> expected atom); keyword-overlap
     retriever mirrors the degraded fs-grep fallback. Extend GOLDEN as the
     corpus grows; daemon-up semantic recall will score differently.

Usage: python3 recall_eval.py [--atoms-dir DIR] [--repos-root DIR] [--today YYYY-MM-DD]
Exit 0 always (report tool). Prints a markdown report.
"""
import argparse, datetime, os, re, sys
from pathlib import Path

VERIFIED_RE = re.compile(r"verified_at:\s*([0-9]{4}-[0-9]{2}-[0-9]{2})")
FNAME_DATE_RE = re.compile(r"([0-9]{4}-[0-9]{2}-[0-9]{2})")
APPLIES_RE = re.compile(r"applies_to:\s*\[([^\]]*)\]")
# code refs like  src/foo/bar.ts:284  or  path/x.rs:10-22  (require a real ext)
CODEREF_RE = re.compile(r"([\w./-]+\.[a-zA-Z]{1,4}):(\d+)(?:-\d+)?")
STALE_DAYS = 90

# Honest seed golden set (query -> substring expected in a relevant atom id/body).
# Extend with real (query, expected) pairs as ground truth accumulates.
GOLDEN = [
    ("paysafe useEffect config id null", "paysafe-debug-useeffect"),
    ("react debugger tracking side effects", "tracking-info-tu-react-debugger"),
    ("kinoa popup OnPopupCloseClick close button", "kinoa"),  # likely a MISS — proves the gap
]


def parse_date(text, fname):
    m = VERIFIED_RE.search(text)
    if m:
        return m.group(1)
    m = FNAME_DATE_RE.search(fname)
    return m.group(1) if m else None


def load_atoms(atoms_dir):
    atoms = []
    for p in Path(atoms_dir).rglob("*.md"):
        if "/atoms/" not in str(p):
            continue
        body = p.read_text(errors="ignore")
        atoms.append({"path": p, "name": p.stem, "body": body,
                      "date": parse_date(body, p.name)})
    return atoms


def freshness(atoms, today):
    buckets = {"<=30d": 0, "31-90d": 0, "91-180d": 0, ">180d": 0, "no-date": 0}
    stale = 0
    for a in atoms:
        if not a["date"]:
            buckets["no-date"] += 1; continue
        try:
            age = (today - datetime.date.fromisoformat(a["date"])).days
        except ValueError:
            buckets["no-date"] += 1; continue
        if age <= 30: buckets["<=30d"] += 1
        elif age <= 90: buckets["31-90d"] += 1
        elif age <= 180: buckets["91-180d"] += 1
        else: buckets[">180d"] += 1
        if age > STALE_DAYS: stale += 1
    return buckets, stale


def code_drift(atoms, repos_root):
    checked = valid = broken = 0
    broken_samples = []
    for a in atoms:
        for ref, line in CODEREF_RE.findall(a["body"]):
            # only check refs that look like repo source paths
            if not ref.startswith(("src/", "crates/", "app/", "lib/", "packages/")):
                continue
            checked += 1
            hits = list(Path(repos_root).glob(f"*/{ref}")) + list(Path(repos_root).glob(ref))
            if hits:
                valid += 1
            else:
                broken += 1
                if len(broken_samples) < 8:
                    broken_samples.append(f"{a['name'][:40]} -> {ref}:{line}")
    return checked, valid, broken, broken_samples


def precision_at_k(atoms, k=3):
    """Keyword-overlap retriever (mirrors fs-grep fallback)."""
    hits = 0
    rows = []
    for query, expected in GOLDEN:
        terms = set(re.findall(r"\w+", query.lower()))
        scored = []
        for a in atoms:
            text = (a["name"] + " " + a["body"]).lower()
            score = sum(1 for t in terms if t in text)
            scored.append((score, a["name"]))
        scored.sort(reverse=True)
        topk = [n for s, n in scored[:k] if s > 0]
        found = any(expected in n for n in topk)
        hits += 1 if found else 0
        rows.append((query, expected, "HIT" if found else "MISS", topk[0] if topk else "-"))
    return hits, len(GOLDEN), rows


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--atoms-dir", default="/Users/nhonh/Documents/ws-memories/geargames")
    ap.add_argument("--repos-root", default="/Users/nhonh/Documents/geargames")
    ap.add_argument("--today", default=datetime.date.today().isoformat())
    args = ap.parse_args()
    today = datetime.date.fromisoformat(args.today)

    atoms = load_atoms(args.atoms_dir)
    print(f"# Recall Eval Report ({args.today})\n")
    print(f"Atoms scanned: **{len(atoms)}**  (dir: {args.atoms_dir})\n")

    b, stale = freshness(atoms, today)
    pct = round(100 * stale / len(atoms), 1) if atoms else 0
    print("## 1. Freshness (by verified_at, not mtime)")
    for k, v in b.items():
        print(f"- {k}: {v}")
    print(f"- **Stale (> {STALE_DAYS}d): {stale}/{len(atoms)} = {pct}%**\n")

    checked, valid, broken, samples = code_drift(atoms, args.repos_root)
    vpct = round(100 * broken / checked, 1) if checked else 0
    print("## 2. Code-drift (cited path:line still on disk?)")
    print(f"- refs checked: {checked} | valid: {valid} | **broken: {broken} ({vpct}%)**")
    for s in samples:
        print(f"  - BROKEN: {s}")
    print()

    hits, total, rows = precision_at_k(atoms)
    print("## 3. precision@3 (seed golden set — degraded fs-grep retriever)")
    print(f"- **{hits}/{total} queries hit**")
    for q, exp, verdict, top in rows:
        print(f"  - [{verdict}] '{q}' -> expect '{exp}' | top1: {top}")
    print("\n_Note: golden set is a seed; precision is on the keyword fallback, "
          "not daemon-up semantic recall. Expand GOLDEN with verified pairs._")


if __name__ == "__main__":
    main()
