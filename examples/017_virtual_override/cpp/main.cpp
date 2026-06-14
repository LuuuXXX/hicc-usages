#include "virtual_override.h"

int main() {
    using namespace virtual_override_ns;
    Triangle t("tri1");
    Pentagon p("pent1");
    std::cout << t.name() << " sides=" << t.sides() << " desc=" << t.describe() << std::endl;
    std::cout << p.name() << " sides=" << p.sides() << " desc=" << p.describe() << std::endl;

    Shape* s = &p;
    std::cout << "via base: " << s->describe() << std::endl;
    return 0;
}
