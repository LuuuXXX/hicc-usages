#pragma once
#include <tuple>
#include <string>
#include <memory>
#include <iostream>

namespace tuple_basic_ns {

// 三元组：(int, std::string, double)
using Triple = std::tuple<int, std::string, double>;

// 返回 unique_ptr<Triple>，hicc 可将返回的不透明对象直接当作 Triple 处理
// （默认 deleter 的 unique_ptr 映射）。
std::unique_ptr<Triple> make_triple(int id, const std::string& name, double score);

// 字段访问器 —— Rust 端通过 cpp! 包装访问 std::get<I>。
int     triple_id(const Triple& t);
std::string triple_name(const Triple& t);
double  triple_score(const Triple& t);

void set_id(Triple& t, int id);
void set_score(Triple& t, double score);

} // namespace tuple_basic_ns
