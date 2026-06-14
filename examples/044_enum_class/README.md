# 044_enum_class — enum_class

## C++ API

`enum class Color : int { Red, Green, Blue };` 强类型枚举。

## hicc 支持方式

⚠️ **限制标注：int 桥接**

`enum class` 类型不可在 `#[cpp(...)]` 签名中命名。值通过 `int` 跨 FFI 传递；Rust 端用 `#[repr(i32)] enum` 镜像枚举并提供 `from_raw`/`name` 转换。

## 关键 AST 字段

| 字段 | 提取方式 |
|------|----------|
| `EnumDecl.name` | 枚举类型名（`Color`） |
| `EnumConstantDecl.name` + `.value` | 枚举常量与整数对应（Red=0, Green=1, Blue=2） |
| `EnumDecl.integerType` | 底层整数类型（`int`） → Rust `#[repr(i32)]` |

## 手工映射步骤

1. AST 抽取 enum 名、底层类型、所有枚举常量。
2. Rust 端生成镜像 `#[repr(i32)] pub enum Color { Red=0, Green=1, Blue=2 }`。
3. C++ 端提供 `to_int_*()` factory 与 `color_name_for_int(int)` 反查函数。
4. Rust 端 `Color::from_raw(i32)` / `Color::name()` 在边界两侧做转换。

## hicc 限制 / 降级

- ❌ `enum class` 类型本身不能直接命名。
- ✅ 通过 int 桥接 + Rust 镜像枚举，类型安全在 Rust 侧恢复。

## 自动化评估

**高**：enum 字段全部从 AST 直接抽取；模板化生成 Rust enum + factory + name wrapper。

## 构建 / 验证

```bash
cd cpp && bash standalone.sh    # or: make    # or: cmake + make
cd rust_hicc && cargo test
../../scripts/verify-one.sh 044_enum_class
```
