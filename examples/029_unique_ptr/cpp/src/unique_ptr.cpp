#include "hicc_usages/unique_ptr.h"
#include <vector>
#include <algorithm>
namespace hicc_usages::unique_ptr {

struct Owner::Impl {
    std::vector<std::unique_ptr<Resource>> items;
};

Resource::Resource(int i) : id_(i) {}
Resource* Resource::create(int id) { return new Resource(id); }
void Resource::free(Resource* self) { delete self; }
int Resource::id() const { return id_; }
void Resource::touch() { ++touches_; }
int Resource::touches() const { return touches_; }

Owner::Owner() : impl_(std::make_unique<Impl>()) {}
Owner::~Owner() = default;
Owner* Owner::create() { return new Owner(); }
void Owner::free(Owner* self) { delete self; }
void Owner::acquire(Resource* r) {
    impl_->items.emplace_back(r);
}
void Owner::release(int id) {
    auto& v = impl_->items;
    v.erase(std::remove_if(v.begin(), v.end(),
        [id](const std::unique_ptr<Resource>& p) { return p->id() == id; }),
        v.end());
}
bool Owner::has(int id) const {
    for (const auto& p : impl_->items) {
        if (p->id() == id) return true;
    }
    return false;
}
std::size_t Owner::count() const { return impl_->items.size(); }

}
