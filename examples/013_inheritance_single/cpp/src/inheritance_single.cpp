#include "hicc_usages/inheritance_single.h"
namespace hicc_usages::inheritance_single {
Animal::Animal(const char* name) : name_(name ? name : "") {}
Animal* Animal::create(const char* name) { return new Animal(name); }
void Animal::free(Animal* self) { delete self; }
const char* Animal::get_name() const { return name_.c_str(); }
int Animal::get_legs() const { return legs_; }
Dog::Dog(const char* name) : Animal(name) {}
Dog* Dog::create(const char* name) { return new Dog(name); }
void Dog::free(Dog* self) { delete self; }
const char* Dog::bark() const { return "Woof!"; }
}
