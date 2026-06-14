# 037_array_basic: AST → hicc 手动映射

## user-ast.json 关键信息

- `ClassTemplateSpecializationDecl array<int, 5>` —— N 是编译期常量
- 业务函数：`array_sum(const std::array<int, 5>&)`、`array_max(...)`、`array_avg(...)`、`fill_array(std::array<int, 5>&, int)`
- 全部以固定 N=5 的 std::array 为参数 / 返回

## hicc 模式：✅ `hicc_std::array<Pod<T>>` 别名 + `make_unique<std::array<T, N>>` 工厂

注意 hicc-std 的 `array<T>` 是 **单参数模板** —— N 隐藏在工厂的实例化里：

```rust
pub class RustArr5 = hicc_std::array<hicc::Pod<i32>>;
#[cpp(func = "std::unique_ptr<std::array<int, 5>> hicc::make_unique<std::array<int, 5>>()")]
pub fn array5_new() -> RustArr5;
```

如果项目里需要多种 N（如 N=5 和 N=10），必须分别 typedef 并使用不同别名。
内置方法可用：`size`、`get`/`get_mut`、`front`/`back`、`as_slice`、`as_slice_mut`。

## 自动化可行性：高

识别 `std::array<T, N>` 的 N 常量即可生成对应工厂。一个 C++ 项目里出现多种 N 时，
为每个不同的 N 生成独立的 Rust 别名 + 工厂（命名约定：`Array{Type}{N}`）。
