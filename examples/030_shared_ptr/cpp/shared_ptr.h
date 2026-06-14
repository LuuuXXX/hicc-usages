#pragma once

#include <memory>

// shared_ptr<T>: same stripping pattern as unique_ptr. Rust receives an
// owned T (refcount-incremented on the C++ side by the wrapper).

class RefCounted {
public:
    RefCounted() { ++count_; }
    ~RefCounted() { --count_; }
    int use_count() const { return count_; }
    static int count() { return count_; }
private:
    static int count_;
};

std::shared_ptr<RefCounted> make_shared_obj();
void shared_obj_free(RefCounted* r);
int  shared_count();
