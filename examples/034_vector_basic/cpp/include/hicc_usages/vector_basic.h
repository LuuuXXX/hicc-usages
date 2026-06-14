#pragma once
#include <cstddef>
#include <iostream>
#include <vector>
namespace hicc_usages::vector_basic {

class IntVector {
public:
    static IntVector* create();
    static void free(IntVector* self);
    void push_back(int v);
    int at(std::size_t idx) const;
    void pop_back();
    std::size_t size() const;
    int sum() const;
    void clear();
private:
    IntVector();
    ~IntVector();
    std::vector<int> data_;
};

}  // namespace hicc_usages::vector_basic
