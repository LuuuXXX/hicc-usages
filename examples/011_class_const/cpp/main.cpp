#include "class_const.h"

int main() {
    using namespace class_const_ns;
    const Temperature freezing(0.0f);
    Temperature boiling(100.0f);

    std::cout << "freezing.value()=" << freezing.value() << " unit=" << freezing.unit() << std::endl;
    std::cout << "freezing.to_fahrenheit()=" << freezing.to_fahrenheit() << std::endl;
    std::cout << "freezing.describe()=" << freezing.describe() << std::endl;

    boiling.convert_to("F");
    std::cout << "boiling after convert: " << boiling.describe() << std::endl;
    boiling.set_value(200.0f);
    std::cout << "boiling after set: " << boiling.describe() << std::endl;
    return 0;
}
