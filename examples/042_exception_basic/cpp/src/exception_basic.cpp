#include "hicc_usages/exception_basic.h"
#include <cstdlib>
#include <cstring>
namespace hicc_usages::exception_basic {

thread_local std::string g_last_error;

const char* last_error() { return g_last_error.c_str(); }
void clear_error() { g_last_error.clear(); }

int safe_divide(int a, int b) {
    try {
        if (b == 0) throw std::invalid_argument("divide by zero");
        return a / b;
    } catch (const std::exception& e) {
        g_last_error = e.what();
        return 0;
    }
}

int safe_at(int* arr, std::size_t size, std::size_t idx) {
    try {
        if (!arr) throw std::invalid_argument("null array");
        if (idx >= size) throw std::out_of_range("index out of bounds");
        return arr[idx];
    } catch (const std::exception& e) {
        g_last_error = e.what();
        return -1;
    }
}

int safe_parse(const char* s) {
    try {
        if (!s) throw std::invalid_argument("null string");
        for (const char* p = s; *p; ++p) {
            if (*p < '0' || *p > '9') {
                if (p == s && *p == '-') continue;
                throw std::invalid_argument("non-numeric character");
            }
        }
        return std::atoi(s);
    } catch (const std::exception& e) {
        g_last_error = e.what();
        return 0;
    }
}

SafeStack::SafeStack(std::size_t cap)
    : data_(new int[cap]), capacity_(cap), count_(0) {}
SafeStack::~SafeStack() { delete[] data_; }
SafeStack* SafeStack::create(std::size_t capacity) { return new SafeStack(capacity); }
void SafeStack::free(SafeStack* self) { delete self; }
void SafeStack::push(int v) {
    try {
        if (count_ >= capacity_) throw std::overflow_error("stack full");
        data_[count_++] = v;
    } catch (const std::exception& e) {
        g_last_error = e.what();
    }
}
int SafeStack::pop() {
    try {
        if (count_ == 0) throw std::underflow_error("stack empty");
        return data_[--count_];
    } catch (const std::exception& e) {
        g_last_error = e.what();
        return -1;
    }
}
int SafeStack::peek() const {
    try {
        if (count_ == 0) throw std::underflow_error("stack empty");
        return data_[count_ - 1];
    } catch (const std::exception& e) {
        g_last_error = e.what();
        return -1;
    }
}
std::size_t SafeStack::size() const { return count_; }
}
