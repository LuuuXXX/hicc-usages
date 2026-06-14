#pragma once
#include <cstddef>
#include <iostream>
#include <functional>
namespace hicc_usages::functional_bind {

class Binder {
public:
    static Binder* create();
    static void free(Binder* self);
    void configure(int base, int step);
    int next();
    int peek() const;
    int calls() const;
private:
    Binder();
    ~Binder();
    int base_;
    int step_;
    int count_;
    std::function<int()> generator_;
};

class Combiner {
public:
    static Combiner* create();
    static void free(Combiner* self);
    void set_first(int (*f)(int));
    void set_pipeline();
    int process(int x) const;
private:
    Combiner();
    ~Combiner();
    std::function<int(int)> first_;
    std::function<int(int)> second_;
};

int double_it(int x);
int add_one(int x);

}  // namespace hicc_usages::functional_bind
