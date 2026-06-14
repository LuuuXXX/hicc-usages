#include "template_class.h"

int main() {
    using namespace template_class_ns;
    Stack<int> s;
    s.push(10);
    s.push(20);
    s.push(30);
    std::cout << "size=" << s.size() << " top=" << s.top() << std::endl;
    s.pop();
    std::cout << "after pop size=" << s.size() << " top=" << s.top() << std::endl;

    Stack<std::string> ss;
    ss.push("hello");
    ss.push("world");
    std::cout << "ss size=" << ss.size() << " top=" << ss.top() << std::endl;
    return 0;
}
