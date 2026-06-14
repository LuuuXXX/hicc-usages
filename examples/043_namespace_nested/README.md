# 043_namespace_nested — namespace_nested

## C++ API

嵌套命名空间 `outer::inner::core` / `outer::inner::util`，使用 `namespace A::B::C { ... }` 语法。

## hicc 支持方式

✅ **直接：命名空间对 FFI 透明**

hicc 的 `#[cpp(func = "...")]` 只关心顶级函数签名。命名空间只影响 C++ 端实现，FFI 端绑定顶级 wrapper 函数即可。

## 关键 AST 字段

| 字段 | 提取方式 |
|------|----------|
| `NamespaceDecl.name` | 拼接成 `outer::inner::core` 等限定名 |
| `FunctionDecl.mangledName` | 实际链接符号（`_ZN5outer5inner4core3addEii`），但 FFI 用 wrapper 函数 |
| `FunctionDecl` 在 `TranslationUnitDecl` 顶层的 | 顶级 wrapper（实际绑定目标） |

## 手工映射步骤

1. AST 中识别嵌套 namespace 下的函数（`outer::inner::core::add`）。
2. **不需要**直接绑定命名空间内的函数；为每个生成顶级 wrapper（`ns_add` 等），wrapper 内部用限定名调用。
3. Rust 端只绑定顶级 wrapper（与 002-005 自由函数模式相同）。

## hicc 限制 / 降级

- ✅ 命名空间完全透明。
- ✅ Wrapper 模式可重用自由函数模板。
- ⚠️ 若想保留命名空间信息，需在 Rust 模块路径上模拟（`mod outer::inner::core`）。

## 自动化评估

**高**：命名空间信息纯 C++ 内部，自动生成顶级 wrapper 完全机械化。

## 构建 / 验证

```bash
cd cpp && bash standalone.sh    # or: make    # or: cmake + make
cd rust_hicc && cargo test
../../scripts/verify-one.sh 043_namespace_nested
```
