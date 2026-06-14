#include "functional_bind.h"
#include <functional>

BindPoint* bind_point_new(int x, int y) { return new BindPoint{x, y}; }
void bind_point_free(BindPoint* p) { delete p; }

namespace {
int add(int a, int b) { return a + b; }
int sub(int a, int b) { return a - b; }
int mul(int a, int b) { return a * b; }
}

int add_bound_10(int x) {
    auto bound = std::bind(add, 10, std::placeholders::_1);
    return bound(x);
}

int mul_bound_3(int x) {
    auto bound = std::bind(mul, std::placeholders::_1, 3);
    return bound(x);
}

int sub_bind_first(int a, int b) {
    using namespace std::placeholders;
    auto bound = std::bind(sub, a, _1);
    return bound(b);
}

int point_x_plus_offset(const BindPoint* p, int offset) {
    auto get_x = [](const BindPoint* pt) { return pt->x; };
    auto bound = std::bind(add, std::bind(get_x, p), offset);
    return bound();
}
