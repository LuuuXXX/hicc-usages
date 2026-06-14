#pragma once
#include <cstddef>
#include <iostream>
namespace hicc_usages::template_specialization {

template<typename T>
struct TypeInfo {
    static const char* name() { return "unknown"; }
    static std::size_t size_of() { return sizeof(T); }
};

template<>
struct TypeInfo<int> {
    static const char* name() { return "int"; }
    static std::size_t size_of() { return sizeof(int); }
};

template<>
struct TypeInfo<double> {
    static const char* name() { return "double"; }
    static std::size_t size_of() { return sizeof(double); }
};

template<>
struct TypeInfo<char> {
    static const char* name() { return "char"; }
    static std::size_t size_of() { return sizeof(char); }
};

}
