#pragma once
#include <string>
#include <typeinfo>
#include <iostream>

namespace typeid_rtti_ns {

class Base {
public:
    virtual ~Base() = default;
    virtual std::string name() const { return "Base"; }
};

class DerivedA : public Base {
public:
    std::string name() const override { return "DerivedA"; }
};

class DerivedB : public Base {
public:
    std::string name() const override { return "DerivedB"; }
};

// RTTI 包装：返回类的 mangled 名
inline const char* type_name_base(const Base& b) {
    return typeid(b).name();
}
inline bool same_type(const Base& a, const Base& b) {
    return typeid(a) == typeid(b);
}
inline bool is_derived_a(const Base& b) {
    return typeid(b) == typeid(DerivedA);
}

} // namespace typeid_rtti_ns
