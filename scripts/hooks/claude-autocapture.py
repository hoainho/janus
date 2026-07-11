#!/usr/bin/env python3
"""
Janus auto-capture hook for Claude Code.

Wired as a Claude Code `Stop` / `SessionEnd` hook. Reads the session transcript
that Claude Code passes on stdin ({"session_id","transcript_path","cwd",...}),
extracts token usage, wall-clock duration, changed files and any ticket key, then
records ONE trace into the Janus durable layer via the existing `harness-cli`
binary — no manual gate typing, no Rust rebuild.

This closes the adoption gap: instead of relying on the agent to remember to run
`harness-cli trace`, every session that ends is captured automatically.

Env:
  HARNESS_DB          path to harness.db (default ~/.harness/harness.db)
  JANUS_CLI           path to harness-cli (default: `harness-cli` on PATH)
  JANUS_CAPTURE_DRYRUN=1   print the commands instead of executing them
  JANUS_CAPTURE_ONLY_TICKET=1  only capture sessions that reference a ticket

Exit code is always 0 (a capture failure must never break a Claude session).
"""
import json
import os
import re
import subprocess
import sys
from datetime import datetime

HOME = os.path.expanduser("~")
DB = os.environ.get("HARNESS_DB", os.path.join(HOME, ".harness", "harness.db"))
CLI = os.environ.get("JANUS_CLI", "harness-cli")
DRYRUN = os.environ.get("JANUS_CAPTURE_DRYRUN") == "1"
ONLY_TICKET = os.environ.get("JANUS_CAPTURE_ONLY_TICKET") == "1"

TICKET_RE = re.compile(r"\b(WIN|WPD)-\d+\b", re.I)
EDIT_TOOLS = {"Edit", "Write", "NotebookEdit", "MultiEdit"}

# Gaps between consecutive messages longer than this are treated as idle
# (session left open) and capped, so `duration` approximates *active* time
# rather than raw wall-clock span. 5 minutes matches the prompt-cache TTL — a
# natural boundary for "still the same working burst".
IDLE_CAP_SECONDS = 300

# Content injected by the harness (not typed by the user). Ticket/summary
# extraction must ignore these or it grabs context noise (e.g. a WIN- ref that
# only appears in the loaded wiki index, not in the user's actual request).
INJECTED_MARKERS = (
    "<system-reminder", "<local-command", "<command-name>", "<command-message>",
    "Caveat: The messages below", "DO NOT respond", "[PROJECT MEMORY]",
    "session-restore", "stdout", "[LLM Wiki",
)


def is_injected(text):
    return any(m in text for m in INJECTED_MARKERS)


def user_text(obj):
    """Return clean user-typed text for a transcript entry, or '' if this turn
    is a tool result / injected content / not a user turn."""
    if obj.get("type") != "user":
        return ""
    msg = obj.get("message") or {}
    content = msg.get("content")
    parts = []
    if isinstance(content, str):
        parts.append(content)
    elif isinstance(content, list):
        for b in content:
            if isinstance(b, dict) and b.get("type") == "text":
                parts.append(b.get("text", ""))
            # tool_result blocks are not user-typed → ignore
    text = " ".join(parts).strip()
    if not text or is_injected(text):
        return ""
    return text


def log(msg):
    sys.stderr.write(f"[janus-autocapture] {msg}\n")


def parse_ts(s):
    if not s:
        return None
    try:
        return datetime.fromisoformat(s.replace("Z", "+00:00"))
    except Exception:
        return None


def scan_transcript(path):
    """Single pass; only json-parse lines that carry signal (keeps huge
    base64 tool-result lines cheap)."""
    tokens = 0
    files_changed = set()
    ticket = None
    first_ts = last_ts = prev_ts = None
    active_seconds = 0
    first_user_text = None
    n_msgs = 0
    errors = 0

    with open(path, "r", encoding="utf-8", errors="ignore") as fh:
        for line in fh:
            n_msgs += 1
            has_usage = '"usage"' in line
            has_tooluse = '"tool_use"' in line or '"name":"Edit"' in line or '"name":"Write"' in line
            has_ticket = ("WIN-" in line.upper()) or ("WPD-" in line.upper())
            has_ts = '"timestamp"' in line
            if not (has_usage or has_tooluse or has_ticket or has_ts):
                continue
            try:
                obj = json.loads(line)
            except Exception:
                continue

            ts = parse_ts(obj.get("timestamp"))
            if ts:
                if first_ts is None:
                    first_ts = ts
                if prev_ts is not None:
                    gap = (ts - prev_ts).total_seconds()
                    if gap > 0:
                        active_seconds += min(gap, IDLE_CAP_SECONDS)
                prev_ts = ts
                last_ts = ts

            msg = obj.get("message") or {}
            usage = msg.get("usage") if isinstance(msg, dict) else None
            if isinstance(usage, dict):
                # Exclude cache_read: it re-counts the same context on every turn
                # and inflates a session to tens of millions. Real spend proxy =
                # input + output + cache_creation.
                tokens += (
                    usage.get("input_tokens", 0)
                    + usage.get("output_tokens", 0)
                    + usage.get("cache_creation_input_tokens", 0)
                )

            content = msg.get("content") if isinstance(msg, dict) else None
            if isinstance(content, list):
                for block in content:
                    if not isinstance(block, dict):
                        continue
                    btype = block.get("type")
                    if btype == "tool_use" and block.get("name") in EDIT_TOOLS:
                        fp = (block.get("input") or {}).get("file_path")
                        if fp:
                            files_changed.add(fp)
                    if btype == "tool_result" and block.get("is_error"):
                        errors += 1

            # ticket + summary ONLY from genuine user-typed text
            utext = user_text(obj)
            if utext:
                if first_user_text is None:
                    first_user_text = utext[:400]
                if ticket is None:
                    m = TICKET_RE.search(utext)
                    if m:
                        ticket = m.group(0).upper()

    wall_clock = None
    if first_ts and last_ts:
        wall_clock = int((last_ts - first_ts).total_seconds())
    duration = int(active_seconds) if first_ts else None

    return {
        "tokens": tokens,
        "files_changed": sorted(files_changed),
        "ticket": ticket,
        "duration": duration,          # active time (idle gaps capped)
        "wall_clock": wall_clock,      # raw first→last span
        "summary": (first_user_text or "").strip(),
        "errors": errors,
        "n_lines": n_msgs,
    }


def already_captured(session_id):
    try:
        out = subprocess.run(
            ["sqlite3", DB,
             f"SELECT 1 FROM trace WHERE notes LIKE '%{session_id}%' LIMIT 1;"],
            capture_output=True, text=True, timeout=10,
        )
        return out.stdout.strip() == "1"
    except Exception:
        return False


def run(cmd):
    if DRYRUN:
        print("DRYRUN:", " ".join(_q(c) for c in cmd))
        return 0
    try:
        r = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
        if r.returncode != 0:
            log(f"cmd failed ({r.returncode}): {r.stderr.strip()[:200]}")
        return r.returncode
    except Exception as e:
        log(f"cmd error: {e}")
        return 1


def _q(s):
    return f'"{s}"' if " " in str(s) else str(s)


def detect_workspace(hook, transcript):
    """Classify the session's workspace so a single shared Janus DB stays
    analysable per-workspace. Live sessions carry `cwd`; backfill infers from
    the transcript's project-dir path (Claude encodes cwd with dashes)."""
    hay = f"{hook.get('cwd', '')} {transcript or ''}"
    if "Documents/personal" in hay or "Documents-personal" in hay:
        return "personal"
    if "Documents/geargames" in hay or "Documents-geargames" in hay:
        return "geargames"
    return "other"


def main():
    raw = sys.stdin.read() if not sys.stdin.isatty() else "{}"
    try:
        hook = json.loads(raw) if raw.strip() else {}
    except Exception:
        hook = {}

    transcript = hook.get("transcript_path") or (sys.argv[1] if len(sys.argv) > 1 else None)
    if not transcript or not os.path.exists(transcript):
        log(f"no transcript ({transcript}); nothing to capture")
        return 0
    # For backfill (no hook stdin), derive a stable session id from the
    # transcript filename (Claude Code names files by session UUID) so
    # idempotency holds per-file.
    session_id = hook.get("session_id") or os.path.splitext(os.path.basename(transcript))[0]

    data = scan_transcript(transcript)

    # only capture sessions that actually did something
    if not data["files_changed"] and not data["ticket"]:
        log("session touched no files and referenced no ticket; skipping")
        return 0
    if ONLY_TICKET and not data["ticket"]:
        log("no ticket and ONLY_TICKET set; skipping")
        return 0

    if not DRYRUN and already_captured(session_id):
        log(f"session {session_id} already captured; skipping")
        return 0

    ticket = data["ticket"]
    workspace = detect_workspace(hook, transcript)
    summary = data["summary"] or f"Claude session {session_id[:8]}"
    summary = re.sub(r"\s+", " ", summary)[:120]
    notes = (f"auto-captured; workspace={workspace}; session_id={session_id}; "
             f"lines={data['n_lines']}; wall_clock_s={data.get('wall_clock')}")

    # ensure a story row exists for the ticket (idempotent-ish: add is safe to
    # attempt; harness-cli rejects dup PK, which we tolerate)
    if ticket:
        run([CLI, "story", "add", "--id", ticket,
             "--title", summary, "--lane", "normal",
             "--notes", "auto-captured story stub (lane defaulted; confirm)"])

    trace_cmd = [
        CLI, "trace",
        "--summary", summary,
        "--agent", "claude",
        "--outcome", "completed" if data["errors"] == 0 else "partial",
        "--notes", notes,
    ]
    if ticket:
        trace_cmd += ["--story", ticket]
    if data["tokens"]:
        trace_cmd += ["--tokens", str(data["tokens"])]
    if data["duration"] is not None:
        trace_cmd += ["--duration", str(data["duration"])]
    if data["files_changed"]:
        trace_cmd += ["--changed", ",".join(data["files_changed"])]
    if data["errors"]:
        trace_cmd += ["--errors", f"{data['errors']} tool errors observed"]

    run(trace_cmd)
    log(f"captured session {session_id[:8]} ws={workspace} "
        f"ticket={ticket} tokens={data['tokens']} "
        f"dur={data['duration']}s files={len(data['files_changed'])}")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Exception as e:  # never break a session
        log(f"fatal (swallowed): {e}")
        sys.exit(0)
