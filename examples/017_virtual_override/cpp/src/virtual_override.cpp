#include "hicc_usages/virtual_override.h"
namespace hicc_usages::virtual_override {
Base* Base::create() { return new Base(); }
void Base::free(Base* self) { delete self; }
const char* Base::name() const { return "Base"; }
int Base::compute(int x) const { return x; }
Derived::Derived(int m) : mult_(m) {}
Derived* Derived::create(int multiplier) { return new Derived(multiplier); }
void Derived::free(Derived* self) { delete self; }
const char* Derived::name() const { return "Derived"; }
int Derived::compute(int x) const { return x * mult_; }
int Derived::multiplier() const { return mult_; }
}
