#include "mutable_member.h"

int main() {
    using namespace mutable_member_ns;
    Query q("user:1");
    std::cout << "exec1: " << q.execute() << std::endl;
    std::cout << "exec2: " << q.execute() << std::endl;
    std::cout << "call_count=" << q.call_count() << std::endl;
    return 0;
}
