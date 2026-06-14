#include "custom_deleter.h"

int main() {
    using namespace custom_deleter_ns;
    auto arr = make_int_array(5);
    for (size_t i = 0; i < 5; ++i) {
        std::cout << "arr[" << i << "]=" << read_at(arr, i) << std::endl;
    }
    std::cout << "bytes_allocated(5)=" << bytes_allocated(5) << std::endl;
    std::cout << "status=" << custom_deleter_status() << std::endl;
    return 0;
}
