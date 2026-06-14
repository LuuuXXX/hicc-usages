#pragma once
#include <cstddef>
#include <iostream>
#include <stdexcept>
namespace hicc_usages::exception_basic {

int safe_divide(int a, int b);
int safe_at(int* arr, std::size_t size, std::size_t idx);
int safe_parse(const char* s);
const char* last_error();
void clear_error();

class SafeStack {
public:
    static SafeStack* create(std::size_t capacity);
    static void free(SafeStack* self);
    void push(int v);
    int pop();
    int peek() const;
    std::size_t size() const;
private:
    SafeStack(std::size_t cap);
    ~SafeStack();
    int* data_;
    std::size_t capacity_;
    std::size_t count_;
};

}  // namespace hicc_usages::exception_basic
