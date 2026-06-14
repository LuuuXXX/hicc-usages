#pragma once

#include <memory>

// unique_ptr<T> return: hicc strips the unique_ptr wrapper — Rust receives
// an owned T (the deleter is wired via destroy= in import_class!).

class Widget {
public:
    explicit Widget(int v) : value_(v) {}
    int value() const { return value_; }
private:
    int value_;
};

// Factory returns unique_ptr<Widget> by value; hicc unwraps it.
std::unique_ptr<Widget> make_widget(int v);
// Deleter used by hicc's destroy= attribute.
void widget_free(Widget* w);
