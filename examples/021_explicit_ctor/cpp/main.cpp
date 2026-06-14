#include "explicit_ctor.h"
#include <iostream>

int main() {
    Fahrenheit* f = fahrenheit_new(212.0);
    Celsius c = convert_to_celsius(*f);
    std::cout << "212F -> " << c.value() << "C" << std::endl;
    fahrenheit_free(f);
    return 0;
}
