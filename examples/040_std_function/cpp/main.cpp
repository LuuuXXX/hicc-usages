#include "std_function.h"

int main() {
    using namespace std_function_ns;
    auto dbl = [](int v) { return v * 2; };
    std::cout << "apply_dbl = " << apply_dbl(dbl, 5) << std::endl;

    auto cb = make_callback([](int v) { return v + 100; });
    std::cout << "cb.invoke(1) = " << cb->invoke(1) << std::endl;
    std::cout << "cb.call_n_times(2, 3) = " << cb->call_n_times(2, 3) << std::endl;

    cb->replace([](int v) { return v * v; });
    std::cout << "after replace cb.invoke(3) = " << cb->invoke(3) << std::endl;

    auto d = make_doubler();
    std::cout << "chain(d, +5, 3) = " << chain(d, [](int v){return v+5;}, 3) << std::endl;
    return 0;
}
