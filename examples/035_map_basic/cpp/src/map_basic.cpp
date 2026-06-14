#include "hicc_usages/map_basic.h"
namespace hicc_usages::map_basic {
IntMap::IntMap() : data_() {}
IntMap::~IntMap() = default;
IntMap* IntMap::create() { return new IntMap(); }
void IntMap::free(IntMap* self) { delete self; }
void IntMap::put(int key, int value) { data_[key] = value; }
bool IntMap::has(int key) const { return data_.find(key) != data_.end(); }
int IntMap::get(int key) const {
    auto it = data_.find(key);
    return it == data_.end() ? -1 : it->second;
}
std::size_t IntMap::size() const { return data_.size(); }
void IntMap::erase(int key) { data_.erase(key); }
int IntMap::sum_values() const {
    int total = 0;
    for (const auto& [k, v] : data_) total += v;
    return total;
}
}
