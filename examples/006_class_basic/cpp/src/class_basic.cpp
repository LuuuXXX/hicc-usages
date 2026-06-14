#include "hicc_usages/class_basic.h"
namespace hicc_usages::class_basic {
Counter* Counter::create() { return new Counter(); }
void Counter::free(Counter* self) { delete self; }
int Counter::get() const { return value_; }
void Counter::increment() { ++value_; }
void Counter::decrement() { --value_; }
void Counter::reset() { value_ = 0; }
}
