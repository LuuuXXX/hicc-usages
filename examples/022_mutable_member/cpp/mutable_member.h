#pragma once

// mutable member: a const method can still modify it. Transparent to FFI.

class Cache {
public:
    Cache() : cached_(0), valid_(false) {}
    int compute(int x) const {
        if (valid_ && x == cached_x_) return cached_;
        cached_x_ = x;
        cached_   = x * x;  // pretend expensive
        valid_   = true;
        return cached_;
    }
    int last_cached() const { return cached_; }
private:
    mutable int cached_;
    mutable int cached_x_;
    mutable bool valid_;
};

Cache* cache_new();
void   cache_free(Cache* c);
