# 008_class_copy: AST → hicc 手动映射记录

## user-ast.json 关键信息

- `CXXRecordDecl Buffer`
- `CXXConstructorDecl Buffer(int, const std::string&)` —— 用户定义
- `CXXConstructorDecl Buffer()` —— 默认
- `CXXConstructorDecl Buffer(const Buffer&)` —— copy ctor
- `CXXConstructorDecl Buffer(Buffer&&) noexcept` —— move ctor
- `CXXMethodDecl operator=(const Buffer&)` —— 赋值
- `size() const`, `tag() const`

## hicc 模式选择

每个 ctor 一个工厂：

| C++ ctor | hicc 工厂模板参数 | Rust 端 |
|---|---|---|
| `Buffer()` | `make_unique<Buffer>()` | `Buffer::default_()` |
| `Buffer(int, const std::string&)` | `make_unique<Buffer, int, const std::string&>(int, const std::string&)` | `Buffer::new(sz, tag)` |
| `Buffer(const Buffer&)` | `make_unique<Buffer, const Buffer&>(const Buffer&)` | `Buffer::clone(&self)` |
| `Buffer(Buffer&&)` | `make_unique<Buffer, Buffer&&>(Buffer&&)` | `Buffer::move_from(other: Self)` |

## 关键代码段（move ctor）

```rust
// move ctor: 接收 Buffer&&，对应 Rust 端的 Self（按值传递）
#[cpp(func = "std::unique_ptr<...> hicc::make_unique<..., Buffer&&>(Buffer&&)")]
pub fn buffer_move(other: Buffer) -> Buffer;
```

## AST 提取要点

- `CXXConstructorDecl` 的 `type.qualType` 区分 copy vs move：含 `&&` 是 move，含 `const T&` 是 copy
- 自动化需识别"5 个特殊成员函数"（default ctor, copy ctor, move ctor, copy assign, move assign, dtor）并选择合适的 Rust 抽象

## 自动化可行性：中

- copy/move ctor 模板可机械生成
- operator= 映射为 `AbiClass::write` 或包装函数需要决策
- 命名约束（Rust 不能与关键字冲突，如 `new` 不能与无参构造冲突）需特例
