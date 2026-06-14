#include "hicc_usages/shared_ptr.h"
#include <map>
#include <memory>
namespace hicc_usages::shared_ptr {

struct Registry::Impl {
    std::map<int, std::shared_ptr<Counter>> items;
};

Counter::Counter(int v) : value_(v) {}
Counter* Counter::create(int initial) { return new Counter(initial); }
void Counter::free(Counter* self) { delete self; }
int Counter::value() const { return value_; }
void Counter::increment() { ++value_; }
void Counter::decrement() { --value_; }

Registry::Registry() : impl_(std::make_shared<Impl>()) {}
Registry::~Registry() = default;
Registry* Registry::create() { return new Registry(); }
void Registry::free(Registry* self) { delete self; }
void Registry::add(int id, Counter* c) {
    impl_->items[id] = std::shared_ptr<Counter>(c, [](Counter* p){ /* deleter */ });
}
int Registry::sum() const {
    int total = 0;
    for (const auto& [k, v] : impl_->items) total += v->value();
    return total;
}
std::size_t Registry::size() const { return impl_->items.size(); }

}
