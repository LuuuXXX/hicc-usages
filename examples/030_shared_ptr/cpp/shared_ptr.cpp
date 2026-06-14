#include "shared_ptr.h"

namespace shared_ptr_ns {

std::shared_ptr<Counter> make_counter(int start) {
    return std::make_shared<Counter>(start);
}

std::shared_ptr<Counter> clone_counter(const std::shared_ptr<Counter>& other) {
    return other;
}

long use_count(const std::shared_ptr<Counter>& p) {
    return p.use_count();
}

int shared_ptr_anchor() { return 30; }
}
