#include "hicc_usages/class_static.h"
namespace hicc_usages::class_static {
int Counter::instance_count_ = 0;
Counter* Counter::create() { ++instance_count_; return new Counter(); }
void Counter::free(Counter* self) { --instance_count_; delete self; }
int Counter::get_instance_count() { return instance_count_; }
void Counter::tick() { ++ticks_; }
int Counter::get_ticks() const { return ticks_; }
}
