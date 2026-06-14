#include "map_basic.h"
#include <iostream>

int main() {
    StringIntMap* m = str_int_map_new();
    m->insert("alice", 30);
    m->insert("bob", 25);
    std::cout << "size=" << m->size()
              << " alice=" << m->get_or("alice", -1)
              << " missing=" << m->get_or("zzz", -1) << std::endl;
    str_int_map_free(m);
    return 0;
}
