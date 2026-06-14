#include "vector_basic.h"
#include <iostream>

int main() {
    IntVector* v = int_vec_new();
    v->push(10);
    v->push(20);
    v->push(30);
    std::cout << "size=" << v->size() << std::endl;
    for (std::size_t i = 0; i < v->size(); ++i) std::cout << v->at(i) << " ";
    std::cout << std::endl;
    int_vec_free(v);
    return 0;
}
