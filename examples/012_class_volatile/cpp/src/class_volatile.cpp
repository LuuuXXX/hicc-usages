#include "hicc_usages/class_volatile.h"
namespace hicc_usages::class_volatile {
Sensor* Sensor::create() { return new Sensor(); }
void Sensor::free(Sensor* self) { delete self; }
int Sensor::read() const { return value_; }
void Sensor::update(int v) { value_ = v; }
int Sensor::read_volatile() const volatile { return value_; }
}
