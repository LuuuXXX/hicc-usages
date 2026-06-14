#include "friend_function.h"

int main() {
    using namespace friend_function_ns;
    Account a("Alice", 1000);
    Account b("Bob", 2500);
    std::cout << a << std::endl;
    std::cout << b << std::endl;
    std::cout << "audit_total(a)=" << audit_total(a) << std::endl;
    std::cout << "audit_total(b)=" << audit_total(b) << std::endl;
    return 0;
}
