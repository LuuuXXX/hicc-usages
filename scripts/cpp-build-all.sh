#!/usr/bin/env bash
# Build all 48 C++ feature libs (standalone + Make + CMake).
# Usage: cpp-build-all.sh
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EXAMPLES="$REPO_ROOT/examples"

fail=0
ok=0
for dir in "$EXAMPLES"/*/; do
    name="$(basename "$dir")"
    [ -f "$dir/cpp/standalone.sh" ] || continue
    if ( cd "$dir/cpp" && bash standalone.sh >/dev/null 2>&1 ); then
        ok=$((ok+1))
    else
        echo "FAIL [standalone] $name"
        fail=$((fail+1))
    fi
done

echo "=== cpp-build-all: $ok ok, $fail failed ==="
[ "$fail" -eq 0 ]
