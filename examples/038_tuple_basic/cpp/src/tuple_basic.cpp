#include "hicc_usages/tuple_basic.h"
namespace hicc_usages::tuple_basic {
Triple::Triple(int a, double b, int c) : data_(a, b, c) {}
Triple::~Triple() = default;
Triple* Triple::create(int a, double b, int c) { return new Triple(a, b, c); }
void Triple::free(Triple* self) { delete self; }
int Triple::first() const { return std::get<0>(data_); }
double Triple::second() const { return std::get<1>(data_); }
int Triple::third() const { return std::get<2>(data_); }
void Triple::set_first(int v) { std::get<0>(data_) = v; }
void Triple::set_second(double v) { std::get<1>(data_) = v; }
void Triple::set_third(int v) { std::get<2>(data_) = v; }
int Triple::sum_ints() const {
    return std::get<0>(data_) + std::get<2>(data_);
}
}
