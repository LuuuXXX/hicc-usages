#include "hicc_usages/union_basic.h"
namespace hicc_usages::union_basic {

Pair::Pair(int a, int b) : first_(a), second_(b) {}
Pair::~Pair() = default;
Pair* Pair::create(int first, int second) { return new Pair(first, second); }
void Pair::free(Pair* self) { delete self; }
int Pair::first() const { return first_; }
int Pair::second() const { return second_; }
int Pair::sum() const { return first_ + second_; }
int Pair::max() const { return first_ > second_ ? first_ : second_; }
}
