#pragma once

// Minimal class with state + methods. Factory + deleter as free functions
// (free-function convention is what hicc's `destroy` attribute consumes).

class Counter {
public:
    Counter() : value_(0) {}
    int  get() const { return value_; }
    void inc()        { ++value_; }
    void reset()      { value_ = 0; }
private:
    int value_;
};

Counter* counter_new();
void     counter_free(Counter* c);
