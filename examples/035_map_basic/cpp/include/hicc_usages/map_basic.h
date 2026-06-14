#pragma once
#include <cstddef>
#include <iostream>
#include <map>
namespace hicc_usages::map_basic {

class IntMap {
public:
    static IntMap* create();
    static void free(IntMap* self);
    void put(int key, int value);
    bool has(int key) const;
    int get(int key) const;
    std::size_t size() const;
    void erase(int key);
    int sum_values() const;
private:
    IntMap();
    ~IntMap();
    std::map<int, int> data_;
};

}  // namespace hicc_usages::map_basic
