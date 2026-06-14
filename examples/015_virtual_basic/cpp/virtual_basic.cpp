#include "virtual_basic.h"

Dog* dog_new(const char* name) { return new Dog(name); }
void dog_free(Dog* d)          { delete d; }
