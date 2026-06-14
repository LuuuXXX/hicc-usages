#!/usr/bin/env bash
# End-to-end verification for a single C++ feature.
#
# Usage: verify-one.sh <NNN>             # picks examples/<NNN>_*
#        verify-one.sh <NNN>_<name>      # explicit dir name
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EXAMPLES="$REPO_ROOT/examples"
EXTRACT="$REPO_ROOT/tools/ast-extract/extract.sh"

if [ "$#" -lt 1 ]; then
    echo "Usage: $0 <NNN> | <NNN>_<name>" >&2
    exit 1
fi

ARG="$1"
if [ -d "$EXAMPLES/$ARG" ]; then
    FEATURE_DIR="$EXAMPLES/$ARG"
else
    # Match by leading number.
    match=$(ls -d "$EXAMPLES/${ARG}_"* 2>/dev/null | head -1 || true)
    if [ -z "$match" ]; then
        echo "error: no example matching '$ARG' in $EXAMPLES" >&2
        exit 2
    fi
    FEATURE_DIR="$match"
fi

FEATURE="$(basename "$FEATURE_DIR")"
NAME="${FEATURE#*_}"

echo "=== [$FEATURE] C++ standalone build ==="
( cd "$FEATURE_DIR/cpp" && bash standalone.sh )

echo "=== [$FEATURE] C++ Make build ==="
( cd "$FEATURE_DIR/cpp" && make clean >/dev/null 2>&1 || true ; make )

echo "=== [$FEATURE] C++ CMake build ==="
( cd "$FEATURE_DIR/cpp" && rm -rf build-cmake && mkdir -p build-cmake \
    && ( cd build-cmake && cmake .. >cmake.log 2>&1 && make ) )

echo "=== [$FEATURE] C++ demo run ==="
if [ -x "$FEATURE_DIR/cpp/build/${NAME}_demo" ]; then
    "$FEATURE_DIR/cpp/build/${NAME}_demo"
elif [ -x "$FEATURE_DIR/cpp/build-cmake/${NAME}_demo" ]; then
    "$FEATURE_DIR/cpp/build-cmake/${NAME}_demo"
fi

echo "=== [$FEATURE] AST extraction ==="
"$EXTRACT" "$FEATURE_DIR/cpp"

echo "=== [$FEATURE] Rust build + test ==="
( cd "$FEATURE_DIR/rust_hicc" && cargo build && cargo test )

echo
echo "[$FEATURE] all stages passed."
