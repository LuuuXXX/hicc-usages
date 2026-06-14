#include "class_volatile.h"

int main() {
    using namespace class_volatile_ns;
    Sensor s(7);
    s.write(100);
    s.write(200);
    std::cout << "id=" << s.id() << " reading=" << s.read() << " counter=" << s.counter() << std::endl;
    s.safe_write(300);
    std::cout << "safe_read=" << s.safe_read() << " counter=" << s.counter() << std::endl;
    return 0;
}
