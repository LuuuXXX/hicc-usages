# 041 functional_bind — AST→hicc 笔记

## AST 关键信息

`functional_bind.h` 在 `functional_bind_ns` 命名空间下声明：
- 3 个被 `std::bind` 包装的 free function（`add`/`multiply`/`subtract`）—— 私有实现，不导出
- 3 个 `make_xxx(int) -> std::function<int(int)>` 工厂：返回 `std::bind` 结果
- 1 个 `apply_bound(std::function<int(int)>, int) -> int`
- 1 个 `compose(outer, inner) -> std::function<int(int)>`
- 类 `BoundAccumulator`：成员 `std::function<int(int)>` + `int base_`，方法 `call_and_accumulate` / `base` / `reset`
- 工厂 `make_accumulator` 返回 `std::unique_ptr<BoundAccumulator>`

clang AST 把 `std::function<int(int)>` 当作普通 `ElaboratedType`（模板特化），不会暴露 `std::bind` 细节。

## hicc 模式选择

`std::bind` 不影响 FFI 形状。C++ 侧把 bind 结果赋值给 `std::function<int(int)>`，hicc 直接按 `hicc::Function<fn(i32) -> i32>` 接收即可（与 040 std::function 完全一致）。Rust 侧通过 `.into()` 把 `hicc::Function` 转为 Rust closure 才能调用。

`BoundAccumulator` 用 `import_class!` + `make_accumulator` 工厂构造。

## 自动化可行性

- 类型映射规则统一：`std::function<int(int)>` ↔ `hicc::Function<fn(i32) -> i32>`
- `std::bind` 在 AST 中仅作为 `make_xxx` 的实现细节，对 filter.py 透明
- 自动可行：filter 跳过 `add`/`multiply`/`subtract`（私有 helper，不导出），导出 `make_*`/`apply_bound`/`compose`/`BoundAccumulator`/`make_accumulator`，等价于 040 的处理
