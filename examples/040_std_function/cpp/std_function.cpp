#include "std_function.h"

namespace std_function_ns {

long Callback::call_n_times(int x, int n) const {
    long s = 0;
    for (int i = 0; i < n; ++i) s += fn_(x);
    return s;
}

int apply_dbl(std::function<int(int)> fn, int x) {
    return fn(x) * 2;
}

std::function<int(int)> make_doubler() {
    return [](int v) -> int { return v * 2; };
}

int chain(std::function<int(int)> f, std::function<int(int)> g, int x) {
    return f(g(x));
}

std::unique_ptr<Callback> make_callback(std::function<int(int)> fn) {
    return std::unique_ptr<Callback>(new Callback(std::move(fn)));
}

int std_function_anchor() { return 40; }

} // namespace std_function_ns
