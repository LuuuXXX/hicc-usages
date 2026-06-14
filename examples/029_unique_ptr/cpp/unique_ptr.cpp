#include "unique_ptr.h"

namespace unique_ptr_ns {

std::unique_ptr<Resource> make_resource(int id, const std::string& name) {
    return std::make_unique<Resource>(id, name);
}

int consume_resource(std::unique_ptr<Resource> r) {
    int id = r->id();
    return id;
}

int unique_ptr_anchor() { return 29; }
}
