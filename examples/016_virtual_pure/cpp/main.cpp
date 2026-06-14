#include "virtual_pure.h"
#include <iostream>

int main() {
    MemoryStorage* s = mem_storage_new();
    s->put("foo", "bar");
    std::cout << "size=" << s->size() << " foo=" << s->get("foo") << std::endl;
    mem_storage_free(s);
    return 0;
}
