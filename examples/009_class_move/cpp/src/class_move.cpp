#include "hicc_usages/class_move.h"
#include <utility>
namespace hicc_usages::class_move {
Owner::Owner(int v) : ptr_(new int(v)) {}
Owner::Owner(Owner&& other) noexcept : ptr_(other.ptr_) { other.ptr_ = nullptr; }
Owner* Owner::create(int value) { return new Owner(value); }
Owner* Owner::take_from(Owner* src) {
    Owner* dst = new Owner(std::move(*src));
    return dst;
}
void Owner::free(Owner* self) { if (self->ptr_) delete self->ptr_; delete self; }
int Owner::get_value() const { return ptr_ ? *ptr_ : -1; }
bool Owner::is_valid() const { return ptr_ != nullptr; }
}
