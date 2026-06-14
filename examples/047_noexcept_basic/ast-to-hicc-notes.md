# 047 noexcept_basic — AST→hicc 笔记

## AST 关键信息

`noexcept_basic.h` 在 `noexcept_basic_ns` 下声明：
- 自由函数：`add_noexcept` / `square_noexcept` / `safe_reciprocal_noexcept` / `compute_constant` (constexpr + noexcept)
- 对比函数：`may_throw` （**不**带 noexcept，会真的抛 `std::runtime_error`）
- 类 `SafeCounter`：构造/`increment`/`get`/`reset`/`describe` 全部 noexcept
- 类 `Buffer`：noexcept 构造 + noexcept move + `size`/`get`/`set`
- 工厂 `make_counter` / `make_buffer` 也标 noexcept

clang AST 把 noexcept 标在 `FunctionDecl::exceptionSpecType` 字段上（`EST_BasicNoexcept`）。

## hicc 模式选择

**完全透明 —— 不需要任何 wrapper 或 C++ 侧修改。**

`noexcept` 只是 C++ 编译期/运行时合约，对 FFI ABI 零影响：
- noexcept 函数不会真正抛异常（抛了也是 `std::terminate`，FFI 无法捕获）
- hicc-build **不感知** `noexcept` 关键字，按普通函数绑定
- Rust 端**不需要** `hicc::Exception<T>` 包装

对比组：`may_throw(int)` 不带 noexcept 且真的抛，必须用 `hicc::Exception<T>` 才能在 Rust 端安全捕获 —— 这恰好印证了 042 的模式（异常是与 noexcept 正交的特性，由真实抛/接行为决定，而不是 noexcept 标签）。

## 项目 memory 提到「047 可能需要 C++ 端微调」 — 实测结论

实测无需任何 C++ 端修改。noexcept 完全透明，hicc-build 直接处理。memory 中那条提示可能是历史 POC 阶段过时信息。

## 自动化可行性

- 完全自动：filter.py 可以从 AST 读 `exceptionSpecType`，但**对 Rust 绑定没影响**
- 实际策略：filter.py 完全忽略 noexcept 关键字，按普通函数生成 Rust 绑定
- 与 001 / 011 等基础示例同档难度
