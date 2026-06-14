# 046 constexpr_basic — AST→hicc 笔记

## AST 关键信息

`constexpr_basic.h` 在 `constexpr_basic_ns` 下声明：
- `struct Constants { static constexpr double PI = 3.14159...; static constexpr int BUFFER_SIZE = 256; ... }`
- `constexpr int square(int)`、`constexpr long factorial(int)`
- `class Circle { constexpr explicit Circle(double); constexpr double radius() const; constexpr double area() const; ... }`
- helper：`compute_area(double)`、`get_pi()`/`get_buffer_size()`/`get_big_number()` 返回 const& 包装

clang AST 把 `constexpr` 标在 `FunctionDecl::isConstexpr` / `VarDecl::isConstexpr` 字段上。
静态 constexpr 成员在 AST 中是 `VarDecl`（in scope of `Constants`），有 `integerLiteral` / `floatingLiteral` 节点给出值。

## hicc 模式选择

三种场景：

1. **静态 constexpr 数据成员**：用 `#[cpp(data = "ns::Class::FIELD")]` 暴露为 `&'static T`。
   hicc-build 用 `EXPORT_CONST_DATA` 包装（参考 datas 例子）。已验证：PI/E（double）、BUFFER_SIZE/MAX_TRIES（int）、BIG_NUMBER（long）。
2. **constexpr 自由函数**：完全透明 —— `#[cpp(func = "...")]`，与 001 等价。
3. **constexpr 方法**：与普通 const 方法一致。hicc-build 不感知 `constexpr` 关键字。

无需任何 wrapper，无 cpp! 块代码，全自动可行。

## 自动化可行性

- 完全自动：
  - filter.py 识别 `VarDecl` 带 `isConstexpr` + `static` → 生成 `#[cpp(data = "qualified_name")]`
  - filter.py 识别 `FunctionDecl`/`CXXMethodDecl` → 忽略 `isConstexpr`，按普通函数生成
  - filter.py 识别 `CXXConstructorDecl` 带 `isConstexpr` → 同普通构造函数
- constexpr 修饰对 FFI 形状零影响，与 001 hello_world 同档难度
