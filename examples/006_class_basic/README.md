# 006_class_basic

最小类：`Counter` 状态 + `get/inc/reset` 方法。factory + deleter 作为自由函数，通过 hicc 的 `destroy = "..."` 属性接到 Rust Drop 路径。

## C++ API

```cpp
class Counter { /* get() const, inc(), reset() */ };
Counter* counter_new();
void     counter_free(Counter*);
```

## 关键 AST 字段

| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 类名 | `CXXRecordDecl.name` | Rust class 名 |
| 公有方法 | `CXXMethodDecl`（在 CXXRecordDecl 内）| 决定 `import_class!` 中的方法列表 |
| 是否 const | `CXXMethodDecl.isConst` | 决定 `&self` 还是 `&mut self` |

## 手工映射步骤

1. 在 C++ 端定义 `Class* <name>_new(...)` factory + `void <name>_free(Class*)` deleter（自由函数）
2. `hicc::cpp! { #include "..." }` 把头文件带入 adapter
3. `import_class!` 中：`#[cpp(class = "Class", destroy = "<name>_free")]`，逐个方法用 `#[cpp(method = "ret f(args) const? volatile?")]`
4. `import_lib!` 中：factory 用 `#[cpp(func = "Class* <name>_new(...)")]`

## hicc 限制 / 降级

- 不能直接绑定 C++ 构造函数，必须通过 factory
- `destroy = "..."` 必须指向**已声明**的符号 — 工厂/deleter 要么在 C++ 头文件中声明，要么在 `hicc::cpp!` 块里定义（且要先于使用它的 `import_class!`）

## 自动化评估

**高**。CXXRecordDecl + CXXMethodDecl 字段都很规整，机械可映射。

## 构建 / 验证

```bash
cd cpp && bash standalone.sh
cd ../rust_hicc && cargo test
# 或 ../../scripts/verify-one.sh 006
```
