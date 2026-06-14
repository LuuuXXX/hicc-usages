#!/usr/bin/env bash
# Standalone build template — produces ../build/lib<name>.a
# Replace lambda_basic and lambda_basic.cpp placeholders when scaffolding.
set -euo pipefail

CXX=${CXX:-g++}
CXX_FLAGS=${CXX_FLAGS:--O2 -std=c++17 -Wall -Wextra -fPIC}
HERE="$(cd "$(dirname "$0")" && pwd)"
BUILD_DIR="$HERE/build"
mkdir -p "$BUILD_DIR"

NAME="lambda_basic"
SOURCES=(lambda_basic.cpp)

OBJ_FILES=()
for src in "${SOURCES[@]}"; do
    obj="$BUILD_DIR/$(basename "$src" .cpp).o"
    echo "[standalone] $CXX $CXX_FLAGS -c $src -o $obj"
    "$CXX" $CXX_FLAGS -I"$HERE" -c "$HERE/$src" -o "$obj"
    OBJ_FILES+=("$obj")
done

echo "[standalone] ar rcs $BUILD_DIR/lib${NAME}.a ${OBJ_FILES[@]}"
ar rcs "$BUILD_DIR/lib${NAME}.a" "${OBJ_FILES[@]}"

# Demo binary (links the freshly built static lib against main.cpp)
if [ -f "$HERE/main.cpp" ]; then
    echo "[standalone] building demo: $BUILD_DIR/${NAME}_demo"
    "$CXX" $CXX_FLAGS -I"$HERE" "$HERE/main.cpp" -L"$BUILD_DIR" -l"$NAME" -o "$BUILD_DIR/${NAME}_demo"
fi

echo "[standalone] done: $BUILD_DIR/lib${NAME}.a"
