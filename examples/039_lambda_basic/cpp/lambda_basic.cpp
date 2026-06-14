#include "lambda_basic.h"

namespace lambda_basic_ns {

int apply_int(int x, std::function<int(int)> fn) {
    return fn(x);
}

std::function<int(int)> make_adder(int add) {
    return [add](int v) -> int { return v + add; };  // 捕获 add
}

std::function<int(int)> compose(std::function<int(int)> f, std::function<int(int)> g) {
    return [f, g](int v) -> int { return f(g(v)); };
}

std::string shout(std::function<std::string(std::string)> fn, const std::string& input) {
    std::string out = fn(input);
    return out + "!";
}

int lambda_basic_anchor() { return 39; }

} // namespace lambda_basic_ns
