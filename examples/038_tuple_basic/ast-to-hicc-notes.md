# 038_tuple_basic: AST → hicc 手动映射

## user-ast.json 关键信息

- `ClassTemplateSpecializationDecl tuple<int, basic_string<char>, double>` —— Triple 别名
- 工厂 `std::unique_ptr<Triple> make_triple(int, const std::string&, double)`
- 字段访问器：`triple_id(const Triple&)`、`triple_name(const Triple&) -> std::string`、`triple_score(const Triple&) -> double`
- 字段修改器：`set_id(Triple&, int)`、`set_score(Triple&, double)`
- 没有 std::get 暴露给 Rust —— 全部封装在访问器内

## hicc 模式：⚠️ std::tuple 不能直接 FFI，需要 cpp! 包装访问器

std::tuple 没有稳定的 ABI 布局，hicc 也没有内置特化。解决方案：

1. **C++ 端**：把字段访问器写成命名空间级自由函数（`triple_id`、`triple_name`、`triple_score`、`set_id`、`set_score`），内部调用 `std::get<I>(t)`。
2. **C++ 端**：工厂 `make_triple` 返回 `std::unique_ptr<Triple>` —— hicc 默认 deleter 映射，把返回值当 Triple 不透明对象处理。
3. **Rust 端**：`import_class!` 声明 Triple 为不透明类（无方法），所有访问通过自由函数。

```rust
#[cpp(class = "tuple_basic_ns::Triple")]
pub class Triple {}  // 不透明

#[cpp(func = "int triple_id(const Triple&)")]
pub fn triple_id(t: &Triple) -> i32;
```

注意：返回 `std::string` 的访问器（triple_name）—— Rust 端按值返回 `hicc_std::string`。

## 自动化可行性：中

需要识别 `std::tuple<Ts...>` 类型，自动为每个字段 `I` 生成访问器函数（`get_I` / `set_I`）。
不同字段类型映射：`int -> i32`、`std::string -> hicc_std::string`、`double -> f64` 等。
工厂自动改为返回 `unique_ptr<Tuple>`，C++ 端写访问器包装。
