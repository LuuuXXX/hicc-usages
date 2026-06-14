#include "operator_overload.h"

int main() {
    using namespace operator_overload_ns;
    Vec2 a(1, 2), b(3, 4);
    Vec2 c = a + b;
    Vec2 d = a - b;
    Vec2 e = a * 2.0f;
    Vec2 f = -a;
    std::cout << "a=" << a << " b=" << b << " a+b=" << c << " a-b=" << d
              << " a*2=" << e << " -a=" << f << std::endl;

    Vec2 g(0, 0);
    g += a;
    std::cout << "g+=a -> " << g << std::endl;

    std::cout << "a==b? " << (a == b ? "yes" : "no") << std::endl;
    std::cout << "a[0]=" << a[0] << " a[1]=" << a[1] << std::endl;
    return 0;
}
