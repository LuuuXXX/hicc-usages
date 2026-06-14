#include "unique_ptr.h"

int main() {
    using namespace unique_ptr_ns;
    auto r = make_resource(1, "res1");
    std::cout << "r id=" << r->id() << " name=" << r->name() << std::endl;

    int consumed = consume_resource(std::move(r));
    std::cout << "consumed id=" << consumed << std::endl;
    return 0;
}
