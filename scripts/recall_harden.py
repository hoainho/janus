#!/usr/bin/env python3
"""recall_harden.py — Freshness/drift filter for ws-memories recall results.

Produces recall_health.json (atom_id -> freshness + drift status) and
exposes freshness_rank() to re-rank nano-brain/omo recall hits conservatively.

Usage:
  python3 recall_harden.py [--atoms-dir DIR] [--repos-root DIR]
                           [--today YYYY-MM-DD] [--retire]

  --retire   Move truly-drifted atoms to <atoms-parent>/_retired/ and write
             a reversible manifest. DEFAULT = dry-run (nothing moved).

Exit 0 always (report tool, never blocks recall path).
"""

import argparse
import datetime
import json
import os
import re
import shutil
import sys
from pathlib import Path

# ---------------------------------------------------------------------------
# Constants
# ---------------------------------------------------------------------------
ATOMS_DIR_DEFAULT = "/Users/nhonh/Documents/ws-memories/geargames"
REPOS_ROOT_DEFAULT = "/Users/nhonh/Documents/geargames"

FRESH_DAYS = 30     # <= 30d  → "fresh",  weight 1.0
AGING_DAYS = 90     # <= 90d  → "aging",  weight 0.8
                    #  > 90d  → "stale",  weight 0.5

# Freshness multipliers used by freshness_rank()
WEIGHT = {"fresh": 1.0, "aging": 0.8, "stale": 0.5}

# Only flag drift for code refs that look like source paths
CODEREF_PREFIXES = ("src/", "crates/", "app/", "lib/", "packages/", "components/")

VERIFIED_RE  = re.compile(r"verified_at:\s*([0-9]{4}-[0-9]{2}-[0-9]{2})")
FNAME_DATE_RE = re.compile(r"([0-9]{4}-[0-9]{2}-[0-9]{2})")
APPLIES_RE   = re.compile(r"applies_to:\s*\[([^\]]*)\]")
CODEREF_RE   = re.compile(r"([\w./ -]+\.[a-zA-Z]{1,5}):(\d+)(?:-\d+)?")

# ---------------------------------------------------------------------------
# Repos that exist locally (resolved once at startup)
# ---------------------------------------------------------------------------
def known_local_repos(repos_root: Path) -> set:
    """Return the set of sub-directory names present under repos_root."""
    if not repos_root.is_dir():
        return set()
    return {p.name for p in repos_root.iterdir() if p.is_dir()}


# ---------------------------------------------------------------------------
# Atom loading
# ---------------------------------------------------------------------------
def load_atoms(atoms_dir: Path) -> list:
    atoms = []
    for p in atoms_dir.rglob("*.md"):
        if "/atoms/" not in str(p):
            continue
        try:
            body = p.read_text(errors="ignore")
        except OSError:
            continue
        atoms.append({
            "path": p,
            "atom_id": _extract_atom_id(body, p.stem),
            "body": body,
            "verified_at": _extract_date(body, p.name),
            "applies_to": _extract_applies_to(body),
        })
    return atoms


def _extract_atom_id(body: str, stem: str) -> str:
    m = re.search(r"^id:\s*(.+)$", body, re.MULTILINE)
    return m.group(1).strip() if m else stem


def _extract_date(body: str, fname: str) -> str | None:
    m = VERIFIED_RE.search(body)
    if m:
        return m.group(1)
    m = FNAME_DATE_RE.search(fname)
    return m.group(1) if m else None


def _extract_applies_to(body: str) -> list:
    m = APPLIES_RE.search(body)
    if not m:
        return []
    raw = m.group(1)
    return [t.strip().strip('"\'') for t in raw.split(",") if t.strip()]


# ---------------------------------------------------------------------------
# Freshness classification
# ---------------------------------------------------------------------------
def classify_freshness(verified_at: str | None, today: datetime.date) -> str:
    if not verified_at:
        return "stale"  # conservative: treat no-date as stale
    try:
        age = (today - datetime.date.fromisoformat(verified_at)).days
    except ValueError:
        return "stale"
    if age <= FRESH_DAYS:
        return "fresh"
    if age <= AGING_DAYS:
        return "aging"
    return "stale"


def age_days(verified_at: str | None, today: datetime.date) -> int | None:
    if not verified_at:
        return None
    try:
        return (today - datetime.date.fromisoformat(verified_at)).days
    except ValueError:
        return None


# ---------------------------------------------------------------------------
# Drift detection — conservative
# ---------------------------------------------------------------------------
def classify_drift(atom: dict, repos_root: Path, local_repos: set) -> str:
    """
    Returns "drifted" | "unknown" | "ok".

    Conservative rules:
      - If applies_to repo is NOT present locally → "unknown" (not drifted).
      - If applies_to repo IS present locally AND a cited path:line file is
        genuinely missing under any sub-repo → "drifted".
      - If no code refs found → "ok" (no claims to verify).
      - If all cited code refs resolve → "ok".
    """
    applies_to_repos = atom["applies_to"]
    body = atom["body"]

    # Gather code refs from the body
    refs = [
        ref for ref, _line in CODEREF_RE.findall(body)
        if ref.startswith(CODEREF_PREFIXES)
    ]

    if not refs:
        return "ok"

    # Determine which applies_to repos are locally present
    present = [r for r in applies_to_repos if r in local_repos]
    absent  = [r for r in applies_to_repos if r and r not in local_repos]

    # If ALL applies_to repos are absent locally → cannot check → "unknown"
    if applies_to_repos and not present:
        return "unknown"

    # Check refs against present repos + bare repos-root
    for ref in refs:
        candidates = (
            list(repos_root.glob(f"*/{ref}"))
            + list(repos_root.glob(ref))
        )
        if not candidates:
            # At least one cited file is missing in a repo we CAN check
            return "drifted"

    return "ok"


# ---------------------------------------------------------------------------
# Build health map
# ---------------------------------------------------------------------------
def build_health(atoms: list, repos_root: Path, local_repos: set,
                 today: datetime.date) -> dict:
    health = {}
    for a in atoms:
        atom_id = a["atom_id"]
        freshness = classify_freshness(a["verified_at"], today)
        drift_status = classify_drift(a, repos_root, local_repos)
        health[atom_id] = {
            "verified_at": a["verified_at"],
            "age_days": age_days(a["verified_at"], today),
            "freshness": freshness,          # "fresh" | "aging" | "stale"
            "drifted": drift_status == "drifted",
            "drift_status": drift_status,    # "ok" | "unknown" | "drifted"
            "applies_to": a["applies_to"],
            "_path": str(a["path"]),
        }
    return health


# ---------------------------------------------------------------------------
# freshness_rank — public contract
# ---------------------------------------------------------------------------
def freshness_rank(hits: list[str], health: dict) -> list[str]:
    """Re-rank recall hit atom_ids by (relevance × freshness_weight).

    Args:
        hits: Ordered list of atom_ids from the recall system (best-first).
              Position encodes relevance: position 0 → score N, position N-1 → 1.
        health: The dict loaded from recall_health.json.

    Returns:
        Re-ranked list with drifted atoms excluded and stale atoms down-weighted.

    Contract:
        - Drifted atoms (drift_status == "drifted") are EXCLUDED entirely.
        - Atoms absent from health are treated as stale, non-drifted (safe fallback).
        - Ties broken by original rank (stable sort).
    """
    n = len(hits)
    ranked = []
    for i, atom_id in enumerate(hits):
        relevance = n - i          # higher = more relevant
        h = health.get(atom_id, {})
        if h.get("drifted", False):
            continue               # exclude truly drifted
        freshness = h.get("freshness", "stale")
        weight = WEIGHT.get(freshness, 0.5)
        score = relevance * weight
        ranked.append((score, i, atom_id))

    ranked.sort(key=lambda x: (-x[0], x[1]))  # desc score, stable on tie
    return [atom_id for _score, _i, atom_id in ranked]


# ---------------------------------------------------------------------------
# Retire (move to _retired/)
# ---------------------------------------------------------------------------
def retire_drifted(atoms: list, health: dict, dry_run: bool) -> list:
    """Move truly-drifted atoms to <atoms-parent>/_retired/ with a manifest.

    Returns list of manifest entries regardless of dry_run.
    """
    manifest = []
    for a in atoms:
        atom_id = a["atom_id"]
        h = health.get(atom_id, {})
        if not h.get("drifted", False):
            continue
        src = a["path"]
        retired_dir = src.parent / "_retired"
        dst = retired_dir / src.name
        manifest.append({
            "atom_id": atom_id,
            "src": str(src),
            "dst": str(dst),
            "verified_at": h["verified_at"],
            "age_days": h["age_days"],
        })
        if not dry_run:
            retired_dir.mkdir(exist_ok=True)
            shutil.move(str(src), str(dst))

    return manifest


# ---------------------------------------------------------------------------
# Reporting
# ---------------------------------------------------------------------------
def print_report(atoms: list, health: dict, manifest: list, dry_run: bool,
                 today: datetime.date) -> None:
    total = len(atoms)
    freshness_counts = {"fresh": 0, "aging": 0, "stale": 0}
    drift_counts = {"ok": 0, "unknown": 0, "drifted": 0}

    for h in health.values():
        freshness_counts[h["freshness"]] = freshness_counts.get(h["freshness"], 0) + 1
        drift_counts[h["drift_status"]] = drift_counts.get(h["drift_status"], 0) + 1

    print(f"# recall_harden.py — Health Summary ({today})")
    print(f"\nAtoms scanned: {total}")
    print()
    print("## Freshness distribution (by verified_at)")
    for bucket, count in freshness_counts.items():
        pct = round(100 * count / total, 1) if total else 0
        weight = WEIGHT[bucket]
        print(f"  {bucket:<8}: {count:>4}  ({pct:>5}%)  [rank weight {weight}]")
    stale_count = freshness_counts["stale"]
    stale_pct = round(100 * stale_count / total, 1) if total else 0
    print(f"\n  >> Stale (>90d): {stale_count}/{total} = {stale_pct}%")
    print(f"     (stale atoms are DOWN-WEIGHTED 0.5×, not excluded)")
    print()
    print("## Drift analysis (code path:line still on disk?)")
    print(f"  ok       (refs found):     {drift_counts['ok']:>4}")
    print(f"  unknown  (repo absent):    {drift_counts['unknown']:>4}  ← NOT flagged as drifted (conservative)")
    print(f"  drifted  (true drift):     {drift_counts['drifted']:>4}  ← excluded from ranked results")
    print()
    if manifest:
        action = "DRY-RUN — would move" if dry_run else "MOVED"
        print(f"## Retire candidates ({action} {len(manifest)} atoms)")
        for m in manifest:
            print(f"  - {m['atom_id']}")
            print(f"    src: {m['src']}")
            print(f"    dst: {m['dst']}")
    else:
        print("## Retire candidates: none (0 truly-drifted atoms)")
    print()
    print("## freshness_rank() contract")
    print("  Input:  ordered list of atom_ids from recall system")
    print("  Output: re-ranked by (relevance × weight), drifted excluded")
    print("  Weights: fresh=1.0 | aging=0.8 | stale=0.5 | drifted=excluded")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------
def main():
    ap = argparse.ArgumentParser(
        description="Build recall_health.json and optionally retire drifted atoms."
    )
    ap.add_argument("--atoms-dir",   default=ATOMS_DIR_DEFAULT)
    ap.add_argument("--repos-root",  default=REPOS_ROOT_DEFAULT)
    ap.add_argument("--today",       default=datetime.date.today().isoformat())
    ap.add_argument("--retire",      action="store_true",
                    help="Actually move drifted atoms (default: dry-run)")
    args = ap.parse_args()

    today       = datetime.date.fromisoformat(args.today)
    atoms_dir   = Path(args.atoms_dir)
    repos_root  = Path(args.repos_root)
    dry_run     = not args.retire

    local_repos = known_local_repos(repos_root)

    atoms  = load_atoms(atoms_dir)
    health = build_health(atoms, repos_root, local_repos, today)

    # Write recall_health.json next to this script
    script_dir = Path(__file__).parent
    health_path = script_dir / "recall_health.json"
    # Strip internal _path key from the JSON output
    health_json = {
        atom_id: {k: v for k, v in h.items() if k != "_path"}
        for atom_id, h in health.items()
    }
    health_path.write_text(json.dumps(health_json, indent=2, default=str))

    manifest = retire_drifted(atoms, health, dry_run=dry_run)

    print_report(atoms, health, manifest, dry_run=dry_run, today=today)

    if not dry_run and manifest:
        manifest_path = script_dir / "recall_retire_manifest.json"
        manifest_path.write_text(json.dumps(manifest, indent=2, default=str))
        print(f"\nManifest written: {manifest_path}")
        print("To reverse: move each 'dst' back to 'src'.")

    print(f"\nHealth file: {health_path}")


if __name__ == "__main__":
    main()
