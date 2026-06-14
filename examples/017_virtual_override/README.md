# 017_virtual_override

Phase 3 (inheritance / polymorphism) — see C++ API in `cpp/` and Rust binding in `rust_hicc/src/lib.rs`.

## 手工映射模式

- **013-017（✅ 直接）**: 派生类独立 `import_class!`，基类方法在派生类上重新声明（FFI 没有继承概念，C++ vtable 透明分发）
- **018（⚠️ 限制标注）**: 菱形虚继承 — 简化为单一具体类（合并中间层接口）

## 关键 AST 字段

| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 基类列表 | `CXXRecordDecl.bases[].type.qualType` | 信息性，提醒"基类方法需在派生类上重新声明" |
| 是否虚函数 | `CXXMethodDecl.isVirtual` | 信息性（vtable 透明） |
| 是否纯虚 | `CXXMethodDecl.isPure` | 抽象类不能直接绑定，只绑派生类 |
| override 列表 | `CXXMethodDecl.overriddenMethods` | 信息性 |

## 自动化评估

**中**。自动化的难点在"哪些基类方法需要在派生类 Rust 类型上重新声明"——这需要追踪继承链。简单的"派生类自身方法列表"自动化高，但合并基类公共方法需要人工判断。

## 构建 / 验证

```bash
../../scripts/verify-one.sh 017
```
