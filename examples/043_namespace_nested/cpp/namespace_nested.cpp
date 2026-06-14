#include "namespace_nested.h"

namespace n1 {
namespace n2 {
namespace n3 {

Foo::Foo() : value_(0) {}
Foo::Foo(int v) : value_(v) {}
int Foo::value() const { return value_; }
void Foo::set_value(int v) { value_ = v; }
std::string Foo::describe() const { return "Foo(" + std::to_string(value_) + ")"; }

std::unique_ptr<Foo> make_foo(int v) {
    return std::unique_ptr<Foo>(new Foo(v));
}

int compute(int x) { return x * x + 1; }

} // namespace n3
} // namespace n2

namespace inner {

Bar::Bar() : name_("anonymous") {}
Bar::Bar(const std::string& name) : name_(name) {}
std::string Bar::name() const { return name_; }
void Bar::rename(const std::string& new_name) { name_ = new_name; }

std::unique_ptr<Bar> make_bar(const std::string& name) {
    return std::unique_ptr<Bar>(new Bar(name));
}

} // namespace inner
} // namespace n1

namespace outer {
namespace deep {
namespace deeper {

int add(int a, int b) { return a + b; }
int triple(int x) { return x * 3; }

} // namespace deeper
} // namespace deep
} // namespace outer

int namespace_nested_anchor() { return 43; }
