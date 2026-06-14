# 042 exception_basic — AST→hicc 笔记

## AST 关键信息

`exception_basic.h` 在 `exception_basic_ns` 下声明：
- `safe_divide(int, int)` —— 除零时抛 `std::invalid_argument`
- `parse_int(const std::string&)` —— 解析失败抛 `std::invalid_argument`
- `nth_char(const std::string&, int)` —— 越界抛 `std::out_of_range`
- `require_even(int)` —— 奇数抛 `std::logic_error`
- 类 `BankAccount`：`deposit`/`withdraw` 方法在错误路径抛异常
- 工厂 `make_account(int) -> std::unique_ptr<BankAccount>`

clang AST 看不到 `throw` 表达式（throw 是函数体的实现细节），只能从注释或运行时获知。所以 `Exception<T>` 必须由开发者**显式声明**。

## hicc 模式选择

`hicc::Exception<T>` 作为返回类型：
- 方法返回 `void` → `hicc::Exception<()>`（如 `deposit`）
- 方法返回 `int` → `hicc::Exception<i32>`（如 `withdraw`/`safe_divide`）
- `std::string` 返回 → `hicc::Exception<hicc_std::string>`（如 `nth_char`）

hicc-build 看到 Rust 端写 `hicc::Exception<T>`，会自动在 C++ 侧用 `EXPORT_EXCEPT_METHOD` / `EXPORT_EXCEPT_MEMBER_METHOD` 包一层 try/catch，把 `std::exception::what()` 编码进 64 字节的 `ExceptionInfo`，原值放进 `ManuallyDrop<T>`。Rust 端 `.ok()` 拿 `Result<T, ExceptionInfo>`，`.what()` 拿错误描述。

注意 `std::string` 参数必须用 `const std::string&` + `&hicc_std::string`（不能按值传递）—— 这是 hicc std::string ABI 的常规约束，与 Exception 机制无关。

## 自动化可行性

- AST 无法自动判断哪些函数会抛异常（throw 在函数体里），需要人工标注
- 一旦标注为 `Exception<T>`，hicc-build 自动生成 C++ 侧 try/catch 包装
- 不建议全自动：filter.py 跳过 `make_account` 之外的所有函数都不合适，因为返回 `hicc::Exception<T>` 是开发者决策
- 半自动可行：filter 给出函数清单，开发者选择哪些标 `Exception<T>`
