#pragma once

// Volatile-qualified methods are rare but hicc supports them via the
// `volatile` qualifier in the method signature.

class VCounter {
public:
    VCounter() : value_(0) {}
    void inc() volatile          { ++value_; }
    int  get() const volatile    { return value_; }
    void reset() volatile        { value_ = 0; }
private:
    int value_;
};

VCounter* vcounter_new();
void      vcounter_free(VCounter* c);
