#pragma once

// constexpr is a compile-time evaluation hint — invisible across FFI.
// hicc just sees the declared C++ function signatures. The fact that
// `sq`/`cube`/`kMagic` are constexpr is fully transparent.

constexpr int kMagic = 7;

constexpr int sq(int x) { return x * x; }
constexpr int cube(int x) { return x * x * x; }

// Runtime helper: gives the linker something to do besides inlining.
int square_plus_magic(int x);
