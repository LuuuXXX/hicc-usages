# 040_std_function: std::function

## user-ast.json 关键信息

- 类 `Callback` 持有 `std::function<int(int)>` 成员，方法 `invoke`（const）、`replace`、`call_n_times`（const）
- 自由函数：`apply_dbl(std::function<int(int)>, int)`、`make_doubler() -> std::function<int(int)>`、`chain(f, g, x)`
- 工厂 `make_callback(std::function<int(int)>) -> std::unique_ptr<Callback>`

## hicc 模式：✅ `std::function<R(Args...)>` ↔ `hicc::Function<fn(Args...) -> R>`

与 039 同样的映射规则。重点演示：std::function 作为**类成员**持久化，通过方法访问。

```rust
#[cpp(method = "void replace(std::function<int(int)>)")]
pub fn replace(&mut self, fn_: hicc::Function<fn(i32) -> i32>);

#[cpp(func = "std::unique_ptr<Callback> make_callback(std::function<int(int)>)")]
pub fn make_callback(fn_: hicc::Function<fn(i32) -> i32>) -> Callback;
```

## ⚠️ hicc 已知约束：C++ 方法 `const` 修饰必须和 Rust 端 `&self` / `&mut self` 严格对应

hicc-build 的 `check_func_with_rust` 规则：
- C++ 方法 `const` 修饰 + Rust `&self` → ✅
- C++ 方法无 `const` + Rust `&mut self` → ✅
- C++ 方法无 `const` + Rust `&self`（不可变） → ❌ "function type is defferent"

**实践**：所有不改成员状态的 C++ 方法都加 `const` 修饰（C++ 良好习惯），Rust 端才能用 `&self`。
本例 `call_n_times` 一开始漏写 `const`，编译失败 → 加上 `const` 后通过。

## 自动化可行性：高

识别 `std::function<R(Args...)>` 自动映射 `hicc::Function<fn(Args...) -> R>`。
自动化工具应**校验**每个类方法的 const 修饰与 Rust 端的 `&self` / `&mut self` 一致性，
对缺 const 的方法主动提示用户在 Rust 端用 `&mut self` 或在 C++ 端补 `const`。
