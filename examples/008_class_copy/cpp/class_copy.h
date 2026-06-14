#pragma once

// Copy semantics exposed via a `box_clone` free function that calls the copy
// ctor. hicc can't bind the copy ctor directly.

class Box {
public:
    explicit Box(int v) : value_(v) {}
    Box(const Box& other) : value_(other.value_) {}
    int  get() const     { return value_; }
    void set(int v)      { value_ = v; }
private:
    int value_;
};

Box* box_new(int v);
Box* box_clone(const Box* src);  // invokes copy ctor
void box_free(Box* b);
