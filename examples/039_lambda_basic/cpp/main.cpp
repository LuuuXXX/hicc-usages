#include "lambda_basic.h"
#include <iostream>

int main() {
    using namespace lambda_basic_ns;
    // 传一个 C++ lambda 给 apply_int
    auto dbl = [](int v) { return v * 2; };
    std::cout << "apply_int(5, x*2) = " << apply_int(5, dbl) << std::endl;

    // 构造一个 C++ lambda（adder）并调用
    auto add10 = make_adder(10);
    std::cout << "add10(7) = " << add10(7) << std::endl;

    // 组合
    auto add5 = make_adder(5);
    auto dbl2 = [](int v) { return v * 2; };
    auto pipe = compose(dbl2, add5);
    std::cout << "compose(x*2, x+5)(3) = " << pipe(3) << std::endl;

    // 字符串 lambda
    auto up = [](std::string s) {
        for (auto& c : s) c = static_cast<char>(std::toupper(static_cast<unsigned char>(c)));
        return s;
    };
    std::cout << "shout = " << shout(up, "hi") << std::endl;
    return 0;
}
