#pragma once
#include <cstddef>
#include <iostream>
#include <functional>
namespace hicc_usages::std_function {

class FuncStore {
public:
    static FuncStore* create();
    static void free(FuncStore* self);
    void set_adder();
    void set_multiplier();
    void set_constant(int c);
    int call(int x) const;
    bool has_func() const;
private:
    FuncStore();
    ~FuncStore();
    std::function<int(int)> func_;
};

class Dispatcher {
public:
    static Dispatcher* create();
    static void free(Dispatcher* self);
    void set_mode(int mode);
    int run(int x) const;
    int run_twice(int x) const;
private:
    Dispatcher();
    ~Dispatcher();
    std::function<int(int)> transform_;
};

}  // namespace hicc_usages::std_function
