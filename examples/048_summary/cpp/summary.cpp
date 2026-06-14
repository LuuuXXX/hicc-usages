#include "summary.h"

Calculator::Calculator(int seed) : seed_(seed) {}
Calculator::~Calculator() {}

int Calculator::base() const { return seed_; }

int Calculator::apply(OpKind op, int x, int y) const {
    switch (op) {
        case OpKind::Add: return seed_ + x + y;
        case OpKind::Sub: return seed_ + x - y;
        case OpKind::Mul: return seed_ + x * y;
        case OpKind::Div:
            if (y == 0) throw std::runtime_error("calc divide by zero");
            return seed_ + x / y;
    }
    throw std::runtime_error("unknown op");
}

std::string Calculator::describe() const {
    return "Calculator(seed=" + std::to_string(seed_) + ")";
}

Calculator* calc_new(int seed) { return new Calculator(seed); }
void calc_free(Calculator* c) { delete c; }

int op_kind_int(int kind) { return kind; }
const char* calc_version() { return "summary-calc-v1"; }

// Thread-local buffer keeps c_str() alive until next call.
const char* calc_describe_c(const Calculator* c) {
    thread_local std::string cache;
    cache = c->describe();
    return cache.c_str();
}
