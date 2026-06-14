#include "map_basic.h"

int main() {
    using namespace map_basic_ns;
    std::map<int, std::string> m;
    put(m, 1, "one");
    put(m, 2, "two");
    put(m, 3, "three");
    std::cout << "size=" << map_size(m) << std::endl;
    std::cout << "key2=" << get_or(m, 2, "?") << std::endl;
    std::cout << "key5=" << get_or(m, 5, "missing") << std::endl;
    std::cout << "sum_keys=" << sum_key_values(m) << std::endl;
    return 0;
}
