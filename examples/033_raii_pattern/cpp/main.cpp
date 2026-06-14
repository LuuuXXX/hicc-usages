#include "raii_pattern.h"

int main() {
    using namespace raii_pattern_ns;
    {
        auto f = open_file(3, "/tmp/raii_demo.txt");
        f->write("hello");
        f->write(" world");
        std::cout << "size=" << f->size() << std::endl;
        long avail = read_file(*f);
        std::cout << "avail=" << avail << std::endl;
    } // unique_ptr dtor → FileHandle dtor → RAII "close"
    std::cout << "scope exited\n";
    return 0;
}
