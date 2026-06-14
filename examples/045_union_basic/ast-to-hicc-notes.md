# 045 union_basic — AST→hicc 笔记

## AST 关键信息

`union_basic.h` 在 `union_basic_ns` 下声明：
- POD `union Value { int i; float f; long l; }`（trivially copyable，size = 8 bytes）
- `enum class Tag : int { Int, Float, Long }` （与 044 同模式）
- 顶层 accessor：`value_as_int`/`value_as_float`/`value_as_long`（按值接 `Value`）
- 顶层 builder：`make_value_int`/`make_value_float`/`make_value_long`（按值返 `Value`）
- 类 `Box`：构造 + 拷贝 + `tag`/`as_*`/`set_*`/`describe`

clang AST 把 `union` 当作 `RecordDecl` + `kind == Union`，每个字段是 `FieldDecl`。

## hicc 模式选择

POD union 可以按值跨 FFI 传（trivially copyable，size = max of members）：

1. **Rust 侧**用一个 `#[repr(C)] struct Value([u8; 8])` 镜像（不暴露字段，当作 opaque 字节块）。
2. **POD union 按值传**：`value_as_int(v: Value) -> i32`、`make_value_int(x) -> Value` —— FFI 时按值传 8 字节。
3. **类内 `Box`**：成员是 `Value`，但所有方法都返回/接收具体类型（int/float/long），不暴露 `Value` 本身。
4. `Tag` enum class 仍按 044 模式包装成 int（`tag_raw -> i32` + Rust 端 `Tag` enum 镜像）。

注意：**非平凡 union**（带 string/带 user-defined ctor）不能用此模式，必须用 `unique_ptr` 包装或写更复杂的 accessor。

## 自动化可行性

- 半自动：filter.py 识别 `UnionDecl`，若是 POD（无 user-defined ctor/dtor），生成 `#[repr(C)] struct X([u8; SIZE])` 镜像 + accessor wrappers
- 类成员 `Value v_;` 不暴露任何字段方法，rust 端只看到具体类型方法 —— 这要求类设计者主动包装，自动化难度大
- 实战策略：union 用 POD-only，rust 端按 opaque bytes；非 POD 用 `unique_ptr<T>` 包一层
