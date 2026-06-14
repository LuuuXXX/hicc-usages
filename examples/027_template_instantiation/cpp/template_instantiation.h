#pragma once

// Explicit instantiation. The template is defined in the header, and the .cpp
// emits the explicit instantiation `template class Stack<int>;` so the symbols
// exist in libtemplate_instantiation.a.

template <typename T>
class Stack {
public:
    Stack() = default;
    void push(T v) { data_ = v; }    // simplified: single-slot
    T    pop() { T v = data_; data_ = T{}; return v; }
    bool empty() const { return data_ == T{}; }
private:
    T data_{};
};
