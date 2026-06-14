#include "template_instantiation.h"
#include <iostream>

int main() {
    Stack<int> s;
    s.push(42);
    std::cout << "popped: " << s.pop() << std::endl;
    return 0;
}
