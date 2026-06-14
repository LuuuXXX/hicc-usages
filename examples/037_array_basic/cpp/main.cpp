#include "array_basic.h"
#include <iostream>

int main() {
    IntArray4* a = int_array4_new();
    for (std::size_t i = 0; i < a->size(); ++i) a->set(i, static_cast<int>(i + 1));
    std::cout << "size=" << a->size() << " sum=" << a->sum() << std::endl;
    int_array4_free(a);
    return 0;
}
