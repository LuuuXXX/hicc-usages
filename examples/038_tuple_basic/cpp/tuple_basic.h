#pragma once

#include <tuple>
#include <string>

// std::tuple<int, std::string, double>. hicc can't bind a tuple directly —
// we provide named accessors (first/second/third) and a "make_tuple" wrapper.

class Triple {
public:
    Triple(int i, std::string s, double d) : data_(i, std::move(s), d) {}
    int         first()  const { return std::get<0>(data_); }
    std::string second() const { return std::get<1>(data_); }
    double      third()  const { return std::get<2>(data_); }
private:
    std::tuple<int, std::string, double> data_;
};

Triple* triple_new(int i, const char* s, double d);
void     triple_free(Triple* t);
