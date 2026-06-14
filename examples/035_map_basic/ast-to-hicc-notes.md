# 035_map_basic: AST → hicc 手动映射

## user-ast.json 关键信息

- `ClassTemplateSpecializationDecl map<int, basic_string<char>, less<int>, allocator<...>>`
- 业务函数：`put(map&, int, const std::string&)`、`get_or(...)`、`map_size(...)`、`sum_key_values(...)`
- 业务函数返回 `std::string`（值，非引用）—— 需要 hicc-std::string 支持按值返回

## hicc 模式：✅ `hicc_std::map<Pod<K>, V>` 别名 + `make_unique` 工厂

1. cpp! 块：`typedef std::map<int, std::string> CppMap;`
2. import_lib! 块：
   - `class RustMap = hicc_std::map<hicc::Pod<i32>, hicc_std::string>;`
   - 工厂 `#[cpp(func = "std::unique_ptr<CppMap> hicc::make_unique<CppMap>()")]`
3. 业务函数（按值返回 std::string）—— 直接绑，hicc_std::string 默认可处理 `std::string` 返回。

Rust 端既可调自定义业务函数（put/get_or/...），也可调 hicc_std::map 内置方法（insert/get/size）。

## 自动化可行性：高

识别 `std::map<K, V>` typedef + 业务函数即可自动生成。
注意 V 为 std::string 时 Rust 端用 `hicc_std::string`；K 为 POD 时包 `hicc::Pod<K>`。
