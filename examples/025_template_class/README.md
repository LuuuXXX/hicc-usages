# 025_template_class

💬 **活跃注入**：类模板无法直接命名。在 `hicc::cpp!` 块内 `using BoxInt = BoxT<int>;` typedef + factory，Rust 端把别名当普通类绑定。

## 手工映射步骤
1. C++ 模板类 `BoxT<T>` 定义在头文件
2. `rust_hicc/src/lib.rs` 顶部 `hicc::cpp! { #include "..." + using BoxInt = BoxT<int>; + factory/deleter inline 函数 }`
3. `import_class! { #[cpp(class = "BoxInt", destroy="...")] pub class BoxInt { ... } }`
4. `import_lib!` 中绑定 factory

## 关键 AST 字段
| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 模板形参 | `ClassTemplateDecl.templateParameters` | 信息性 |
| 字段/方法 | 同普通类（在特化节点下） | 同普通类 |

## 自动化评估
**中**。需要识别"哪些类型参数需要实例化" — 通常靠人工判断。

## 构建 / 验证
`../../scripts/verify-one.sh 025`
