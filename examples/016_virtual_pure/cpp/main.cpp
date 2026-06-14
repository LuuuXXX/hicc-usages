#include "virtual_pure.h"

int main() {
    using namespace virtual_pure_ns;
    InMemoryStorage s;
    s.put("a", "1");
    s.put("b", "2");
    std::cout << "a=" << s.get("a") << " size=" << s.size() << std::endl;
    s.dump();
    s.remove("a");
    std::cout << "after remove a: size=" << s.size() << std::endl;

    Storage* p = &s;
    p->put("c", "3");
    p->dump();
    return 0;
}
