#include "placement_new.h"

int main() {
    using namespace placement_new_ns;
    Buffer buf(sizeof(Payload));
    Payload* p = place_payload(buf, 42);
    std::cout << "p value=" << p->value() << std::endl;
    p->set(99);
    std::cout << "p value=" << p->value() << std::endl;
    destroy_payload(p);
    return 0;
}
