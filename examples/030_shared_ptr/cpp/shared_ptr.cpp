#include "shared_ptr.h"

int RefCounted::count_ = 0;

std::shared_ptr<RefCounted> make_shared_obj() {
    return std::make_shared<RefCounted>();
}

void shared_obj_free(RefCounted* r) { delete r; }
int  shared_count() { return RefCounted::count(); }
