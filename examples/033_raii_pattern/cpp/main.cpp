#include "raii_pattern.h"
#include <iostream>

int main() {
    {
        Lock* l = lock_new(7);
        std::cout << "id=" << l->id() << " locked=" << l->is_locked() << std::endl;
        lock_free(l);  // RAII release fires
    }
    return 0;
}
