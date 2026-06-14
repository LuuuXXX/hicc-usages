#pragma once
#include <tuple>
#include <string>
#include <memory>
#include <iostream>

namespace tuple_basic_ns {

// A 3-tuple: (int, std::string, double)
using Triple = std::tuple<int, std::string, double>;

// Returns unique_ptr<Triple> so hicc can treat the returned opaque object
// directly as Triple (default-deleter unique_ptr mapping).
std::unique_ptr<Triple> make_triple(int id, const std::string& name, double score);

// Field accessors — Rust 端通过 cpp! 包装访问 std::get<I>。
int     triple_id(const Triple& t);
std::string triple_name(const Triple& t);
double  triple_score(const Triple& t);

void set_id(Triple& t, int id);
void set_score(Triple& t, double score);

} // namespace tuple_basic_ns
