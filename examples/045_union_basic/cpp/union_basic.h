#pragma once

// Anonymous/named union types are not nameable across FFI. We expose
// a class (ValueBox) that *contains* a union internally and provides
// typed accessors; Rust only sees the box + setter/getter methods.

class ValueBox {
public:
    enum class Tag : int { Int = 0, Float = 1 };

    ValueBox();
    ~ValueBox();

    void set_int(int v);
    void set_float(float v);
    int   get_int() const;
    float get_float() const;
    int   tag() const;

private:
    Tag tag_;
    union {
        int   as_int;
        float as_float;
    };
};

ValueBox* value_box_new();
void value_box_free(ValueBox* b);
