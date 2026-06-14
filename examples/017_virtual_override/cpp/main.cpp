#include "virtual_override.h"
#include <iostream>

int main() {
    InfoLogger*  i = info_logger_new();
    ErrorLogger* e = error_logger_new();
    std::cout << i->format("hello") << std::endl;
    std::cout << e->format("oops")  << std::endl;
    logger_free_info(i);
    logger_free_error(e);
    return 0;
}
