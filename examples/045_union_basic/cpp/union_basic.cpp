#include "union_basic.h"
#include <charconv>
#include <cstdio>

namespace union_basic_ns {

Box::Box(int x)   : tag_(Tag::Int)   { v_.i = x; }
Box::Box(float x) : tag_(Tag::Float) { v_.f = x; }
Box::Box(long x)  : tag_(Tag::Long)  { v_.l = x; }
Box::Box(const Box& other) : v_(other.v_), tag_(other.tag_) {}

Tag Box::tag() const { return tag_; }
int   Box::as_int()   const { return v_.i; }
float Box::as_float() const { return v_.f; }
long  Box::as_long()  const { return v_.l; }

void Box::set_int(int x)   { v_.i = x; tag_ = Tag::Int; }
void Box::set_float(float x) { v_.f = x; tag_ = Tag::Float; }
void Box::set_long(long x)  { v_.l = x; tag_ = Tag::Long; }

std::string Box::describe() const {
    char buf[64];
    switch (tag_) {
        case Tag::Int:
            std::snprintf(buf, sizeof(buf), "int(%d)", v_.i);
            break;
        case Tag::Float:
            std::snprintf(buf, sizeof(buf), "float(%g)", static_cast<double>(v_.f));
            break;
        case Tag::Long:
            std::snprintf(buf, sizeof(buf), "long(%ld)", v_.l);
            break;
    }
    return std::string(buf);
}

Box make_box_int(int x)   { return Box(x); }
Box make_box_float(float x) { return Box(x); }
Box make_box_long(long x)  { return Box(x); }

int union_basic_anchor() { return 45; }

} // namespace union_basic_ns
