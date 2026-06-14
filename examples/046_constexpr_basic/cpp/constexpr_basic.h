#pragma once
#include <string>
#include <iostream>

namespace constexpr_basic_ns {

// 静态 constexpr 数据成员 —— 暴露为 &'static
struct Constants {
    static constexpr double PI = 3.14159265358979;
    static constexpr double E  = 2.71828182845905;
    static constexpr int    BUFFER_SIZE = 256;
    static constexpr int    MAX_TRIES   = 5;
    static constexpr long   BIG_NUMBER = 9000000000L;
};

// constexpr 函数（注意：constexpr 在运行期是透明的）
constexpr int square(int x) { return x * x; }
constexpr long factorial(int n) {
    long r = 1;
    for (int i = 2; i <= n; ++i) r *= i;
    return r;
}

// 带 constexpr 方法的类 —— 按普通方法包装
class Circle {
public:
    constexpr explicit Circle(double radius) : radius_(radius) {}
    constexpr double radius() const { return radius_; }
    constexpr double area() const { return Constants::PI * radius_ * radius_; }
    void set_radius(double r) { radius_ = r; }
private:
    double radius_;
};

// 辅助函数：在运行期通过 constexpr 方法计算面积
double compute_area(double radius);

// 返回 constexpr 静态数据 const 引用的包装
const double& get_pi();
const int& get_buffer_size();
const long& get_big_number();

} // namespace constexpr_basic_ns
