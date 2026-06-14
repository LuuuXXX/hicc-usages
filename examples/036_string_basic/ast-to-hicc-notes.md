# 036_string_basic: AST → hicc 手动映射

## user-ast.json 关键信息

- `ClassTemplateSpecializationDecl basic_string<char, char_traits<char>, allocator<char>>` （std::string）
- 业务函数全部以 `std::string` 为参数 / 返回值：`greet`、`to_upper`、`concat`、`string_length`、`contains_substring`
- 没有 typedef 别名 —— 直接用 `std::string`

## hicc 模式：✅ 直接绑业务函数（参数 / 返回 = std::string）

hicc-std 已为 `std::string` 内置实例化（`hicc_std::string::from(c"...")`、`hicc_std::string::with_cstr` 等）。
所以不需要在 cpp! 块写 typedef，import_lib! 直接绑业务函数即可：

```rust
#[cpp(func = "std::string ns::greet(const std::string&)")]
pub fn greet(name: &hicc_std::string) -> hicc_std::string;
```

引用 `class string = hicc_std::string;` 让 hicc-std::string 在 ClassRef 转换路径上更顺，但本例没用到方法。

Rust 端转 CStr 的方式：`unsafe { CStr::from_ptr(s.c_str()) }.to_str()`。

## 自动化可行性：高

识别 `std::string` 直接生成 `hicc_std::string` 别名，按值返回 / 引用参数都自动处理。
