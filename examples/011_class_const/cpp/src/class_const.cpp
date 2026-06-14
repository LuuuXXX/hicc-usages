#include "hicc_usages/class_const.h"
namespace hicc_usages::class_const {
Value::Value(int v) : value_(v) {}
Value* Value::create(int initial) { return new Value(initial); }
void Value::free(Value* self) { delete self; }
int Value::get() const { return value_; }
void Value::set(int v) { value_ = v; }
void Value::add(int delta) { value_ += delta; }
}
