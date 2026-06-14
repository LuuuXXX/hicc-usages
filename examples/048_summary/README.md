# 048_summary — summary

## C++ API

综合示例：`Calculator` 类（factory + free pair）+ `OpKind` enum class + `apply()` 抛 `std::runtime_error` + `describe()` 返回 `std::string` + 顶级 `calc_version()`。

## hicc 支持方式

✅ **直接综合**：四种模式叠加，每个模式与独立示例完全一致。

| 子特性 | 来源示例 | 模式 |
|--------|----------|------|
| 类 + factory/free | 006-009 | `import_class!` + `import_lib!` factory |
| `OpKind` enum | 044 | int 桥接 + Rust 镜像 enum |
| `throw` | 042 | `hicc::Exception<T>` 返回 |
| `std::string` 返回 | 036 | `thread_local std::string` cache + `const char*` wrapper |

## 关键 AST 字段

合并 006/036/042/044 的提取规则；本例验证规则在同一 TU 中可叠加。

## 手工映射步骤

1. 解析 `Calculator` 类 → 套用 006 类绑定模板。
2. 解析 `apply()` 抛异常 → 套用 042 `Exception<T>` 返回模板。
3. 解析 `OpKind` enum → 套用 044 int 桥接模板（apply 用 `i32`）。
4. 解析 `describe()` 返回 `std::string` → 套用 036 thread_local 缓存 + `const char*` wrapper。

## hicc 限制 / 降级

- ✅ 所有子特性独立工作，组合无副作用。
- ⚠️ thread_local 缓存使 `describe()` 在多线程下结果会被最后一次调用覆盖（单线程测试 OK）。

## 自动化评估

**高**：每个子特性的提取规则独立可机械化；本例证明模板组合可行。

## 构建 / 验证

```bash
cd cpp && bash standalone.sh    # or: make    # or: cmake + make
cd rust_hicc && cargo test
../../scripts/verify-one.sh 048_summary
```
