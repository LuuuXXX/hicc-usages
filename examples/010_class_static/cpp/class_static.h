#pragma once
#include <string>
#include <iostream>

namespace class_static_ns {

class Counter {
public:
    Counter() : id_(++s_next_id_), count_(0) {
        ++s_alive_;
        std::cout << "Counter() id=" << id_ << std::endl;
    }
    ~Counter() {
        --s_alive_;
        std::cout << "~Counter() id=" << id_ << std::endl;
    }

    void inc() { ++count_; }
    int count() const { return count_; }
    int id() const { return id_; }

    // 静态成员函数
    static int alive() { return s_alive_; }
    static int next_id() { return s_next_id_; }
    static const std::string& species() { return s_species_; }

    // 静态成员变量
    static int s_total_created;

private:
    int id_;
    int count_;

    static int s_alive_;
    static int s_next_id_;
    static const std::string s_species_;
};

} // namespace class_static_ns
