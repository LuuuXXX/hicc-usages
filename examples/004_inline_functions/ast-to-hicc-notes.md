# 004_inline_functions: AST → hicc 手动映射记录

## user-ast.json 关键信息

- `int square(int)` (inline)
- `int cube(int)` (inline)
- `int factorial(int)` (constexpr inline)

## hicc 模式选择

inline / constexpr 是编译期提示，对 FFI **完全透明**。Rust 端按普通函数导出。但 inline 函数如果只在头文件中定义而无 out-of-line 拷贝，链接时可能找不到符号——通过在 .cpp 中显式实例化或加 `__attribute__((used))` 解决。本例 `inline_ns::inline_anchor()` 起到 anchor 作用。

## 自动化可行性：高

零特殊处理。AST 中 `FunctionDecl` 的 `storageClass = "inline"` 标记可忽略。
