# 002_function_overload: AST → hicc 手动映射记录

## user-ast.json 关键信息

4 个 `add` 重载：
- `int add(int, int)` → `add_int`
- `double add(double, double)` → `add_double`
- `std::string add(const std::string&, const std::string&)` → `add_string`
- `int add(int, int, int)` → `add_three`

## hicc 模式选择

Rust 没有同名不同签名的重载，**必须给每个 C++ 重载起独立 Rust 名字**。C++ 侧不改，全靠 `#[cpp(func = "...")]` 的签名区分。

## 关键代码段

```rust
#[cpp(func = "int overload_ns::add(int, int)")]
pub fn add_int(a: i32, b: i32) -> i32;

#[cpp(func = "std::string overload_ns::add(const std::string&, const std::string&)")]
pub fn add_string(a: &string, b: &string) -> string;
```

## 注意点

- `std::string` 参数：在 `import_lib!` 块顶部 `class string = hicc_std::string;` 起别名，参数用 `&string`，返回值用 `string`
- 字符串返回值的 `c_str()` 给出 `*const i8`，调用方用 `CStr::from_ptr` 转 Rust 字符串

## 自动化可行性：高

- AST 中 `FunctionDecl` 的 `type.qualType` 直接给完整签名
- 自动按参数类型生成 Rust 别名（int/double/string/arity 等组合）
- 命名冲突是机械可解的（type-based mangling）
