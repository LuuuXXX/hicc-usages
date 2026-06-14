#pragma once
#include <string>
#include <iostream>

namespace constexpr_basic_ns {

// Static constexpr data members — exposed as &'static
struct Constants {
    static constexpr double PI = 3.14159265358979;
    static constexpr double E  = 2.71828182845905;
    static constexpr int    BUFFER_SIZE = 256;
    static constexpr int    MAX_TRIES   = 5;
    static constexpr long   BIG_NUMBER = 9000000000L;
};

// constexpr functions (note: constexpr is transparent at runtime)
constexpr int square(int x) { return x * x; }
constexpr long factorial(int n) {
    long r = 1;
    for (int i = 2; i <= n; ++i) r *= i;
    return r;
}

// Class with constexpr methods — wrap as normal method
class Circle {
public:
    constexpr explicit Circle(double radius) : radius_(radius) {}
    constexpr double radius() const { return radius_; }
    constexpr double area() const { return Constants::PI * radius_ * radius_; }
    void set_radius(double r) { radius_ = r; }
private:
    double radius_;
};

// Helper to compute area at runtime via the constexpr method
double compute_area(double radius);

// Wrapper returning const reference to constexpr static data
const double& get_pi();
const int& get_buffer_size();
const long& get_big_number();

} // namespace constexpr_basic_ns
