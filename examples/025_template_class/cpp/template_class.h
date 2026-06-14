#pragma once

// Class template. Rust can't name `Box<int>` directly — we typedef it to a
// concrete C++ type via a `using` alias inside the hicc::cpp! block, then
// bind the alias as a normal class.

template <typename T>
class BoxT {
public:
    explicit BoxT(T v) : value_(v) {}
    T get() const { return value_; }
    void set(T v) { value_ = v; }
private:
    T value_;
};
