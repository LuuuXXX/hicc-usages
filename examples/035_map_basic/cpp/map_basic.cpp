#include "map_basic.h"

namespace map_basic_ns {

void put(std::map<int, std::string>& m, int key, const std::string& val) {
    m[key] = val;
}

std::string get_or(const std::map<int, std::string>& m, int key, const std::string& def) {
    auto it = m.find(key);
    if (it == m.end()) return def;
    return it->second;
}

size_t map_size(const std::map<int, std::string>& m) {
    return m.size();
}

long sum_key_values(const std::map<int, std::string>& m) {
    long s = 0;
    for (const auto& kv : m) s += kv.first;
    return s;
}

int map_basic_anchor() { return 35; }

} // namespace map_basic_ns
