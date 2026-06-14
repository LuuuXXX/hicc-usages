#pragma once
#include <cstddef>
#include <iostream>
#include <tuple>
namespace hicc_usages::tuple_basic {

class Triple {
public:
    static Triple* create(int a, double b, int c);
    static void free(Triple* self);
    int first() const;
    double second() const;
    int third() const;
    void set_first(int v);
    void set_second(double v);
    void set_third(int v);
    int sum_ints() const;
private:
    Triple(int a, double b, int c);
    ~Triple();
    std::tuple<int, double, int> data_;
};

}  // namespace hicc_usages::tuple_basic
