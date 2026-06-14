#include "hicc_usages/functional_bind.h"
namespace hicc_usages::functional_bind {

Binder::Binder() : base_(0), step_(1), count_(0), generator_(nullptr) {}
Binder::~Binder() = default;
Binder* Binder::create() { return new Binder(); }
void Binder::free(Binder* self) { delete self; }
void Binder::configure(int base, int step) {
    base_ = base;
    step_ = step;
    count_ = 0;
    using namespace std::placeholders;
    generator_ = std::bind(
        [](int b, int s, int& c) { int v = b + s * c; ++c; return v; },
        base, step, std::ref(count_));
}
int Binder::next() { return generator_ ? generator_() : -1; }
int Binder::peek() const { return base_ + step_ * count_; }
int Binder::calls() const { return count_; }

Combiner::Combiner() : first_(nullptr), second_(nullptr) {}
Combiner::~Combiner() = default;
Combiner* Combiner::create() { return new Combiner(); }
void Combiner::free(Combiner* self) { delete self; }
void Combiner::set_first(int (*f)(int)) {
    first_ = f;
}
void Combiner::set_pipeline() {
    second_ = [this](int x) {
        if (!first_) return x;
        return first_(x) * 2;
    };
}
int Combiner::process(int x) const {
    return second_ ? second_(x) : x;
}

int double_it(int x) { return x * 2; }
int add_one(int x) { return x + 1; }
}
