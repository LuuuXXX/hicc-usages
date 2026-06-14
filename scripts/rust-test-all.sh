#!/usr/bin/env bash
# Run cargo test for all 48 rust_hicc crates. Requires ../cpp/build/lib*.a
# present (run cpp-build-all.sh first).
# Usage: rust-test-all.sh
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EXAMPLES="$REPO_ROOT/examples"

fail=0
ok=0
for dir in "$EXAMPLES"/*/; do
    name="$(basename "$dir")"
    [ -f "$dir/rust_hicc/Cargo.toml" ] || continue
    # Build cpp first so the static lib is fresh.
    ( cd "$dir/cpp" && bash standalone.sh >/dev/null 2>&1 ) || true
    if ( cd "$dir/rust_hicc" && cargo test --quiet >/dev/null 2>&1 ); then
        ok=$((ok+1))
        echo "PASS $name"
    else
        echo "FAIL $name"
        fail=$((fail+1))
    fi
done

echo "=== rust-test-all: $ok ok, $fail failed ==="
[ "$fail" -eq 0 ]
