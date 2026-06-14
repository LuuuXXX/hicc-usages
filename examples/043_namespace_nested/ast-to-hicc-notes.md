# 043 namespace_nested — AST→hicc 笔记

## AST 关键信息

`namespace_nested.h` 在多层嵌套命名空间下声明：
- `n1::n2::n3::Foo`：构造、`value` / `set_value` / `describe`，工厂 `make_foo`，自由函数 `compute`
- `n1::inner::Bar`：`name` / `rename`，工厂 `make_bar`
- `outer::deep::deeper::add` / `triple`：自由函数

clang AST 的 `qualifiedName` 字段直接给出带完整命名空间的类型名，对 hicc-build 完全透明。

## hicc 模式选择

直接用完整带命名空间的签名：`#[cpp(class = "n1::n2::n3::Foo")]`、`#[cpp(func = "int n1::n2::n3::compute(int)")]`。命名空间嵌套层数对 FFI 没有影响 —— hicc-build 直接用字符串解析 C++ 全名并查找符号。

`std::string` 仍然用 `hicc_std::string` 接收（按 `const std::string&` + `&hicc_std::string`）。

## 自动化可行性

- 完全自动：filter.py 直接把 `qualifiedName` 当成 `#[cpp(class = "...")]` / `#[cpp(func = "...")]` 字符串即可
- 命名空间层级无需任何特殊处理，与 001 hello_world 同档难度
- 多个命名空间分布在同一个 .h 也无影响
