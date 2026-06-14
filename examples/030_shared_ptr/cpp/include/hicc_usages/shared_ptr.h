#pragma once
#include <cstddef>
#include <iostream>
#include <memory>
namespace hicc_usages::shared_ptr {

class Counter {
public:
    static Counter* create(int initial);
    static void free(Counter* self);
    int value() const;
    void increment();
    void decrement();
private:
    explicit Counter(int v);
    int value_;
};

class Registry {
public:
    static Registry* create();
    static void free(Registry* self);
    void add(int id, Counter* c);
    int sum() const;
    std::size_t size() const;
private:
    Registry();
    ~Registry();
    struct Impl;
    std::shared_ptr<Impl> impl_;
};

}  // namespace hicc_usages::shared_ptr
