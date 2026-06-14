#pragma once
#include <memory>
#include <string>
#include <iostream>

namespace shared_ptr_ns {

class Counter {
public:
    Counter(int start) : count_(start) {}
    int value() const { return count_; }
    void increment() { ++count_; }
    void decrement() { --count_; }
private:
    int count_;
};

std::shared_ptr<Counter> make_counter(int start);
std::shared_ptr<Counter> clone_counter(const std::shared_ptr<Counter>& other);
long use_count(const std::shared_ptr<Counter>& p);

} // namespace shared_ptr_ns
