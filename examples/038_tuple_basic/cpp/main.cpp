#include "tuple_basic.h"

int main() {
    using namespace tuple_basic_ns;
    auto t = make_triple(7, "alice", 9.5);
    std::cout << "id=" << triple_id(*t) << std::endl;
    std::cout << "name=" << triple_name(*t) << std::endl;
    std::cout << "score=" << triple_score(*t) << std::endl;
    set_id(*t, 8);
    set_score(*t, 8.8);
    std::cout << "after update id=" << triple_id(*t) << " score=" << triple_score(*t) << std::endl;
    return 0;
}
