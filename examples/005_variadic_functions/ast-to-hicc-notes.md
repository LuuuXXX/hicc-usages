# 005_variadic_functions: AST → hicc 手动映射记录

## user-ast.json 关键信息

- `int sum_ints(int, ...)` —— C 风格变长
- `void log_line(const char*, ...)` —— printf 风格
- `int sum_va(int, va_list)` —— va_list 风格（hicc 文档推荐）

## hicc 模式选择

reference.md 明确：
1. C 风格 `...` → Rust `unsafe fn f(arg0, ...)`，调用方式 `f(arg0)(extra_args)`（函数返回函数指针）
2. `va_list` 作为最后参数 → 同样模式，但前面参数可以是 C++ 类类型

本例只导出 C 风格 `...` 的两个函数（sum_va 是中间实现，不暴露给 Rust）。

## 关键代码段

```rust
#[cpp(func = "int variadic_ns::sum_ints(int, ...)")]
pub unsafe fn sum_ints(count: i32, ...) -> i32;

// 调用：
let total = unsafe { sum_ints(3)(10, 20, 30) };
```

## AST 提取要点

- `ParmVarDecl` 的 `type.qualType` 包含 `...`（clang 把它编码为最后一个特殊参数）
- 自动化识别：最后一个参数是 `...` → 用 `unsafe fn(arg0, ...) -> R` 模式

## 自动化可行性：中

- 模式机械
- 但 va_list 模式需要识别「最后参数是 va_list」并选择不同签名模板
- 调用方式特殊（返回函数指针），文档需说明
