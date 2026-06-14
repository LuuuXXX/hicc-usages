# 026_template_specialization: AST → hicc 手动映射

## user-ast.json 关键信息

- `ClassTemplateDecl ClassName<T>`（主模板）
- `ClassTemplateSpecializationDecl ClassName<int>` / `ClassName<double>` / `ClassName<std::string>`
- 每个特化有 `name()` / `describe(arg)` 接口，签名各不相同

## hicc 模式：✅ **直接 import_lib!**

每个特化对外都暴露**自由函数**（`int_name()`、`int_describe(int)`、`double_name()`、`string_describe(const std::string&)`）。
返回 `const char*` 在 Rust 端用 `*const i8` 接，再用 `CStr::from_ptr` 取出字符串。

```rust
#[cpp(func = "const char* template_specialization_ns::int_name()")]
pub fn int_name() -> *const i8;

#[cpp(func = "std::string template_specialization_ns::string_describe(const std::string&)")]
pub fn string_describe(s: &hicc_std::string) -> hicc_std::string;
```

string 返回需 `class string = hicc_std::string;` 在 import_lib 内声明。

## 自动化可行性：高

模板特化 + 自由函数包装已经扁平，自动化只需识别 CXXMethodDecl / FunctionDecl 列表生成 import_lib! 项。注意区分 `const char*` 返回 vs `std::string` 返回。
