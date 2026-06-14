#pragma once
namespace hicc_usages::class_basic {
class Counter {
    int value_ = 0;
public:
    static Counter* create();
    static void free(Counter* self);
    int get() const;
    void increment();
    void decrement();
    void reset();
};
}
