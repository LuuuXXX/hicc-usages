#include "template_specialization.h"
#include <iostream>

int main() {
    std::cout << "int     -> " << type_name_int()     << std::endl;
    std::cout << "bool    -> " << type_name_bool()    << std::endl;
    std::cout << "generic -> " << type_name_generic() << std::endl;
    return 0;
}
