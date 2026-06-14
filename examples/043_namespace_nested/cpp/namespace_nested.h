#pragma once
#include <string>
#include <memory>
#include <iostream>

namespace n1 {
namespace n2 {
namespace n3 {

class Foo {
public:
    Foo();
    explicit Foo(int v);
    int value() const;
    void set_value(int v);
    std::string describe() const;
private:
    int value_;
};

std::unique_ptr<Foo> make_foo(int v);
int compute(int x);

} // namespace n3
} // namespace n2

namespace inner {
class Bar {
public:
    Bar();
    explicit Bar(const std::string& name);
    std::string name() const;
    void rename(const std::string& new_name);
private:
    std::string name_;
};

std::unique_ptr<Bar> make_bar(const std::string& name);
} // namespace inner

} // namespace n1

// Top-level namespace alias for testing
namespace outer {
namespace deep {
namespace deeper {

int add(int a, int b);
int triple(int x);

} // namespace deeper
} // namespace deep
} // namespace outer
