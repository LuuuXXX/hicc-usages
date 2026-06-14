# 034_vector_basic: AST → hicc 手动映射

## user-ast.json 关键信息

- `ClassTemplateSpecializationDecl vector<int, allocator<int>>`
- 业务函数：`vector_sum(const std::vector<int>&)`、`vector_avg(...)`、`build_vector(...)`
- 头文件 `<vector>` 引入完整模板

## hicc 模式：✅ `hicc_std::vector<Pod<T>>` 别名 + `make_unique` 工厂

1. cpp! 块：`typedef std::vector<int> CppVec;` —— C++ 侧实例化类型。
2. import_lib! 块：
   - `class RustVec = hicc_std::vector<hicc::Pod<i32>>;` —— Rust 端别名（T 必须是 Pod）
   - `#[cpp(func = "std::unique_ptr<CppVec> hicc::make_unique<CppMap>()")]` 工厂返回 RustVec
3. Rust 端通过 hicc_std::vector 内置方法（push_back / size / as_slice / front / back 等）操作容器。

注意：hicc-std 容器作为函数参数时，Rust 端类型写 `&RustVec`（引用），对应 C++ 端 `const std::vector<int>&`。

## 自动化可行性：高

只要识别 `std::vector<T>` 类型的 typedef + 业务函数签名即可自动生成 import_lib 块。
POD 模板参数（int/double/float/...）必须包 `hicc::Pod<T>`；类模板参数（std::string）保持原样。
