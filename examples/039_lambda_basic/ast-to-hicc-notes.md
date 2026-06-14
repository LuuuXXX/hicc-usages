# 039_lambda_basic: AST → hicc 手动映射

## user-ast.json 关键信息

- 4 个高阶函数：
  - `apply_int(int, std::function<int(int)>) -> int` —— 接受 lambda
  - `make_adder(int) -> std::function<int(int)>` —— 返回捕获 lambda
  - `compose(f, g) -> std::function<int(int)>` —— 高阶组合
  - `shout(std::function<std::string(std::string)>, const std::string&) -> std::string`
- 没有 lambda 表达式直接出现在头文件签名里 —— 一律通过 `std::function<R(Args...)>` 跨边界

## hicc 模式：✅ `std::function<R(Args...)>` → `hicc::Function<fn(Args...) -> R>`

```rust
#[cpp(func = "int apply_int(int, std::function<int(int)>)")]
pub fn apply_int(x: i32, fn_: hicc::Function<fn(i32) -> i32>) -> i32;

#[cpp(func = "std::function<int(int)> make_adder(int)")]
pub fn make_adder(add: i32) -> hicc::Function<fn(i32) -> i32>;
```

转换路径：
- Rust 闭包 → `hicc::Function<...>`：`.into()`（前提是闭包类型匹配 `fn(...) -> R`）
- `hicc::Function<...>` → Rust 可调用：`.into()` 反向

注意：构造 `hicc_std::string` 并在闭包内返回（`std::function<std::string(std::string)>`）会触发段错误，
原因可能是 hicc::Function 对返回 by-value 类对象的 ABI 还原不完整。**生产实践**：尽量让 lambda 签名
只用 POD（int / double / 浮点）类型，避免在 Rust → C++ 方向构造 `hicc_std::string`。
C++ 端构造 string 没问题（参考 035/036 的 get_or / greet）。

## 自动化可行性：高（POD lambda）

识别 `std::function<R(Args...)>` 自动映射到 `hicc::Function<fn(Args...) -> R>`，
按 Args / R 类型递归应用 POD / hicc_std::string 映射规则。
对返回 by-value 类对象的 std::function 标记为"需要 C++ 端中间 wrapper"以规避已知 bug。
