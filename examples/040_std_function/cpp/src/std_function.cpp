#include "hicc_usages/std_function.h"
namespace hicc_usages::std_function {
FuncStore::FuncStore() : func_(nullptr) {}
FuncStore::~FuncStore() = default;
FuncStore* FuncStore::create() { return new FuncStore(); }
void FuncStore::free(FuncStore* self) { delete self; }
void FuncStore::set_adder() {
    int offset = 10;
    func_ = [offset](int x) { return x + offset; };
}
void FuncStore::set_multiplier() {
    int factor = 3;
    func_ = [factor](int x) { return x * factor; };
}
void FuncStore::set_constant(int c) {
    func_ = [c](int) { return c; };
}
int FuncStore::call(int x) const {
    return func_ ? func_(x) : -1;
}
bool FuncStore::has_func() const { return static_cast<bool>(func_); }

Dispatcher::Dispatcher() : transform_(nullptr) {}
Dispatcher::~Dispatcher() = default;
Dispatcher* Dispatcher::create() { return new Dispatcher(); }
void Dispatcher::free(Dispatcher* self) { delete self; }
void Dispatcher::set_mode(int mode) {
    switch (mode) {
        case 0: transform_ = [](int x) { return x; }; break;
        case 1: transform_ = [](int x) { return x + 1; }; break;
        case 2: transform_ = [](int x) { return x * 2; }; break;
        default: transform_ = [](int) { return 0; }; break;
    }
}
int Dispatcher::run(int x) const {
    return transform_ ? transform_(x) : -1;
}
int Dispatcher::run_twice(int x) const {
    if (!transform_) return -1;
    return transform_(transform_(x));
}
}
