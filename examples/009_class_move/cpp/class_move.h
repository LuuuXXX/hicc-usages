#pragma once

// Move semantics: an rvalue-qualified method (&&) is exposed in Rust as
// `fn take(self, ...) -> ...` (consumes the receiver). After the call, the
// C++ side marks the moved-from state; hicc wires Rust's Drop to no-op on
// the moved-from instance via the destroy attribute.

class Resource {
public:
    explicit Resource(int v) : value_(v), valid_(true) {}

    // && qualified: can only be called on an rvalue (Rust self = by-value).
    int consume_value() && {
        int v = value_;
        value_ = -1;
        valid_ = false;
        return v;
    }

    int  peek() const { return value_; }
    bool is_valid() const { return valid_; }

private:
    int  value_;
    bool valid_;
};

Resource* resource_new(int v);
void      resource_free(Resource* r);
