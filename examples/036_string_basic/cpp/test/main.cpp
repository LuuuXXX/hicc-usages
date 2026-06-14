#include "hicc_usages/string_basic.h"
#include <cassert>
#include <cstring>
#include <iostream>
int main() {
    using namespace hicc_usages::string_basic;
    StringBuf* s = StringBuf::create_from("Hello");
    s->append(", World!");
    assert(s->length() == 13);
    assert(strcmp(s->c_str(), "Hello, World!") == 0);
    assert(s->equals("Hello, World!"));
    assert(s->find("World") == 7);
    const char* sub = s->substring(0, 5);
    assert(strcmp(sub, "Hello") == 0);
    StringBuf::free(s);
    std::cout << "[string_basic] C++ test OK" << std::endl;
    return 0;
}
