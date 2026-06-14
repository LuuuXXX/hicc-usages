#include "hicc_usages/mutable_member.h"
namespace hicc_usages::mutable_member {
Cache* Cache::create() { return new Cache(); }
void Cache::free(Cache* self) { delete self; }
int Cache::get_value(int key) const {
    ++access_count_;
    if (key < 0 || key >= 16) return -1;
    return values_[key];
}
void Cache::set_value(int key, int value) {
    if (key >= 0 && key < 16) values_[key] = value;
}
int Cache::access_count() const { return access_count_; }
}
