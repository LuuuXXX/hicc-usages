#include "virtual_diamond.h"

int main() {
    using namespace virtual_diamond_ns;
    IOCombo c("dev1");
    c.write(99);
    std::cout << c.id() << " category=" << c.category()
              << " read=" << c.read() << " last_out=" << c.last_output() << std::endl;
    return 0;
}
