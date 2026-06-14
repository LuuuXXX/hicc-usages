# 020_friend_function: AST → hicc 手动映射

## user-ast.json 关键信息

- `CXXRecordDecl Account`
- `CXXConstructorDecl Account(const std::string&, long)`
- member: `owner() const`, `balance() const`
- 自由函数（friend）: `audit_total(const Account&)`, `operator<<(std::ostream&, const Account&)`

## hicc 模式

friend 在 Rust 侧透明 — friend function 实际是普通命名空间级自由函数，直接 import_lib! 绑定。无需 `friend` 关键字处理。

```rust
#[cpp(func = "long friend_function_ns::audit_total(const friend_function_ns::Account&)")]
pub fn audit_total(a: &Account) -> i64;
```

## 自动化可行性：高

`FunctionDecl` 在 namespace 顶层 + 参数类型涉及某 class → 即 friend-style 函数，按普通自由函数绑定即可。
