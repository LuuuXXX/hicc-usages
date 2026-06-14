#include "union_basic.h"

ValueBox::ValueBox() : tag_(Tag::Int), as_int(0) {}
ValueBox::~ValueBox() {}

void ValueBox::set_int(int v) { tag_ = Tag::Int; as_int = v; }
void ValueBox::set_float(float v) { tag_ = Tag::Float; as_float = v; }
int ValueBox::get_int() const { return as_int; }
float ValueBox::get_float() const { return as_float; }
int ValueBox::tag() const { return static_cast<int>(tag_); }

ValueBox* value_box_new() { return new ValueBox(); }
void value_box_free(ValueBox* b) { delete b; }
