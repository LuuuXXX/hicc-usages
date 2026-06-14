# 044 enum_class — AST→hicc 笔记

## AST 关键信息

`enum_class.h` 在 `enum_class_ns` 下声明：
- `enum class Color { Red, Green, Blue }` （默认 int，0/1/2）
- `enum class Status : int { Active = 10, Inactive = 20, Pending = 30 }` （显式值）
- 自由函数：`color_to_int`/`color_from_int`/`color_name`/`color_parse`、`status_to_int`/`status_from_int`
- 类 `Light`：构造接受 `Color`，方法 `current()` 返 `Color`，`set(Color)`，`brightness()` 返 int
- 工厂 `make_light(Color)`

clang AST 中 `enum class` 的每个 enumerator 有 `integerLiteral` 节点给出值，能直接拿到（`Active = 10`）。

## hicc 模式选择

`enum class` 不能直接 FFI —— hicc 端无法表达 C++ scoped enum 的 ABI（带类型/作用域）。所以：

1. **C++ 侧**写 `*_to_int` / `*_from_int` 转换器作为枚举和 `int` 之间的桥。
2. **Rust 侧**定义对应的 `#[repr(i32)] pub enum` 作为镜像（仅类型糖），FFI 时仍按 `i32` 传。
3. 对类方法返回/接受 enum 的，**在 `hicc::cpp!` 块**写 inline 包装（如 `light_current_int`/`light_set_int`），把 enum 转成 int 后再跨 FFI。
4. Rust 端用安全包装函数 `Light::current() -> Color` 把 `i32` 转回 `Color` 镜像。

## 自动化可行性

- 半自动可行：filter.py 识别 `enum class X` AST 节点 → 自动生成：
  - C++ 侧的 `X_to_int` / `X_from_int` 包装（必须写入 `cpp!` 块或单独头文件）
  - Rust 侧的 `#[repr(i32)] pub enum X` 镜像 + 转换函数
- 类方法签名需重写：把任何 `Color` 返回/参数都改成 `int`，方法名加 `_int` 后缀
- 工作量中等：filter.py 已能识别 enum class，关键是 C++ 侧 wrapper 的代码生成模板化
