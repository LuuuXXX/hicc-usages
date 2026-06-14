#include "function_overload.h"

int main() {
    std::cout << "add(int,int) = " << overload_ns::add(1, 2) << std::endl;
    std::cout << "add(double,double) = " << overload_ns::add(1.5, 2.5) << std::endl;
    std::cout << "add(string,string) = " << overload_ns::add(std::string("foo"), std::string("bar")) << std::endl;
    std::cout << "add(int,int,int) = " << overload_ns::add(1, 2, 3) << std::endl;
    return 0;
}
