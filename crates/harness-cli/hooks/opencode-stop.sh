#!/bin/sh
# opencode Stop hook for harness-cli
# Runs eval harness on changed skill files when opencode session ends
# Requires: OPENCODE_CHANGED_FILES env var (NUL-separated)

set -e

discover_changed_skills() {
    local files="${OPENCODE_CHANGED_FILES:-}"
    if [ -z "$files" ]; then return 0; fi
    printf '%s' "$files" | tr '\0\n' '\n\n' | awk -F'/' '
        /\.opencode\/skills\// {
            for (i=1; i<=NF; i++) if ($i == "skills" && (i+1)<=NF) { print $(i+1); next }
        }
    ' | sort -u
}

changed_skills=$(discover_changed_skills)
if [ -z "$changed_skills" ]; then
    exit 0
fi

exit_aggregate=0
for skill in $changed_skills; do
    [ -z "$skill" ] && continue
    echo "[eval-harness] stop-hook: running evals for changed skill: $skill" >&2
    harness-cli eval run --skill="$skill" --trigger=stop-hook || exit_aggregate=$?
done
exit "$exit_aggregate"
