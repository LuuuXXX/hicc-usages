#pragma once

// Partial / full specialization. We provide a primary template, a full
// specialization for `bool`, and a wrapper that lets Rust call either.

template <typename T>
struct TypeName {
    static const char* get() { return "generic"; }
};

template <>
struct TypeName<bool> {
    static const char* get() { return "bool"; }
};

template <>
struct TypeName<int> {
    static const char* get() { return "int"; }
};

// Rust binds these wrappers; each calls the appropriate specialization.
const char* type_name_int();
const char* type_name_bool();
const char* type_name_generic();
