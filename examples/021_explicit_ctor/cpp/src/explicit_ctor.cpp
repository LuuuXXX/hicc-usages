#include "hicc_usages/explicit_ctor.h"
namespace hicc_usages::explicit_ctor {
Distance::Distance(int meters) : meters_(meters) {}
Distance* Distance::create_from_meters(int m) { return new Distance(m); }
Distance* Distance::create_from_feet(int f) { return new Distance(static_cast<int>(f * 0.3048)); }
void Distance::free(Distance* self) { delete self; }
int Distance::meters() const { return meters_; }
int Distance::feet() const { return static_cast<int>(meters_ / 0.3048); }
}
