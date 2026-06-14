#pragma once

// ⚠️ hicc's #[cpp(method = "...")] does not accept `noexcept`.
// This is the ONLY example in the project where the C++ side is modified
// to accommodate hicc: the original `int add(int, int) noexcept` is
// demoted to `int add(int, int)` so the signature can be expressed in
// hicc's attribute. (No-throw guarantees still hold at runtime; we just
// can't say so in the cross-FFI signature.)

struct SafeAdder {
    int base;
    // Original: int add(int x) const noexcept;   ← hicc cannot bind
    // Adjusted:                                       ↓ no `noexcept`
    int add(int x) const;
    int sub(int x) const;
    int combined(int x, int y) const;
};

SafeAdder* safe_adder_new(int base);
void safe_adder_free(SafeAdder* s);
