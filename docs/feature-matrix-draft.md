# 特性矩阵设计稿（P2）

> 输入：`../cpp2rust-demo/examples/` 的 48 个 C++ 特性。
> 输出：48 行 × (特性 / 作用 / hicc 支持路径 / 关键代码模式) + 6 批分组。
> 用途：作为 P3-P5 实施的输入，每行的「hicc 模式」直接决定该特性的 `rust_hicc/src/lib.rs` 用哪个宏。

## 表 1：48 特性 × hicc 支持路径

| # | 特性 | C++ 作用 | hicc 支持 | hicc 模式 | 关键代码片段 |
|---|---|---|---|---|---|
| 001 | hello_world | 最简单的全局函数 | ✅ | `import_lib!` + `#[cpp(func = "void hello_world()")]` | `fn hello_world();` |
| 002 | function_overload | 函数重载 | ✅ | `import_lib!` 多个 `#[cpp(func = ...)]` + Rust 端改名 | `fn add_i(a: i32) -> i32; fn add_s(s: &str) -> i32;` |
| 003 | default_args | 缺省参数 | ✅ | `import_lib!` 省略尾部参数 | `#[cpp(func = "int greet(const char*, int)")] fn greet(name: *const i8) -> i32;` |
| 004 | inline_functions | 内联函数 | ✅ | 透明（编译期内联，FFI 不感知） | 同 001 |
| 005 | variadic_functions | 变长参数 | ✅ | `import_lib!` + `unsafe fn f(fmt: *const i8, ...)` | `unsafe fn log(fmt: *const i8, ...);` |
| 006 | class_basic | 基本类 | ✅ | `import_class!` + `#[cpp(class)]` + `#[cpp(method)]` | `class Foo { fn bar(&self); }` |
| 007 | class_constructor | 构造函数 | ✅ | 工厂函数 + Rust 关联函数包装 | `Foo::new() -> Self` |
| 008 | class_copy | 拷贝构造 | ✅ | 工厂函数 + AbiClass.write() | `Foo::copy(&other) -> Self` |
| 009 | class_move | 移动构造 | ✅ | C++ `T&&` 自动映射为 `T` | `Foo::from_moved(T) -> T` |
| 010 | class_static | 静态成员 | ✅ | `import_lib!` + `#[method(class, name)]` | `Foo::count() -> usize` |
| 011 | class_const | const 成员 | ✅ | `#[cpp(method = "void foo() const")]` → `&self` | `fn foo(&self);` |
| 012 | class_volatile | volatile 成员 | ⚠️ 部分 | C++ 保留 volatile，Rust 用 `&mut T`，文档说明 | 包装函数擦除 volatile |
| 013 | inheritance_single | 单继承 | ✅ | `class D: B { ... }` 语义级 | `class Derived: Base { ... }` |
| 014 | inheritance_multiple | 多继承 | ✅ | 同上 | `class D: B1, B2 { ... }` |
| 015 | virtual_basic | 虚函数 | ✅ | `#[interface]` 转为 Trait | `#[interface] class Foo { fn speak(&self); }` |
| 016 | virtual_pure | 纯虚 | ✅ | 同 015 + `@make_proxy<T>()` | Rust impl trait |
| 017 | virtual_override | override | ✅ | 同 015 | 同 015 |
| 018 | virtual_diamond | 菱形继承 | ✅ | `class D: virtual B` 语法 | 包装层处理 |
| 019 | operator_overload | 操作符重载 | ❌ 直接 | `cpp!` 块写 `add(a, b)` 包装 | `static T add(const T&, const T&);` |
| 020 | friend_function | 友元函数 | ✅ | 友元在 Rust 侧透明，正常导出友元函数 | 同 001 |
| 021 | explicit_ctor | explicit 构造 | ✅ | 工厂函数包装，explicit 标记对 FFI 透明 | `Foo::from_int(i32) -> Foo` |
| 022 | mutable_member | mutable 成员 | ✅ | C++ mutable 在 const 内修改，FFI 透明 | 包装 accessor |
| 023 | typeid_rtti | RTTI typeid | ❌ 直接 | `cpp!` 块写 `type_name(const T&)` 包装 | `static const char* name(const T&);` |
| 024 | template_function | 模板函数 | ✅ | `#[cpp(func = "T foo<T>(...)")]` 完整签名 | 模板函数声明 |
| 025 | template_class | 模板类 | ✅ | `#[cpp(class = "template<...> T<...>")]` | 同 reference.md vector |
| 026 | template_specialization | 偏特化 | ✅ | 多个具体类型别名 + 工厂 | `class VecI32 = std::vector<int>;` |
| 027 | template_instantiation | 显式实例化 | ✅ | cpp! 块 `template class Foo<int>;` + 工厂 | 显式实例化 |
| 028 | variadic_template | 变参模板 | ✅ | 写具体实例化 + 多个工厂 | 同 027 |
| 029 | unique_ptr | std::unique_ptr | ✅ | 默认 deleter 自动映射为 T | `fn make() -> Foo;` |
| 030 | shared_ptr | std::shared_ptr | ✅ | 默认 deleter 自动映射为 T | 同 029 |
| 031 | custom_deleter | 自定义 deleter | ✅ | `hicc::unique_ptr<T>` / `hicc::shared_ptr<T>` | 显式 unique_ptr 类型 |
| 032 | placement_new | placement new | ✅ | `hicc::placement_new<T>(buf, len, args)` | 见 hicc-examples/placement_new |
| 033 | raii_pattern | RAII | ✅ | Drop 自动调用析构 | `Foo::new()` + scope |
| 034 | vector_basic | std::vector | ✅ | `hicc_std::vector` | typedef + 工厂 |
| 035 | map_basic | std::map | ✅ | `hicc_std::map` | 同上 |
| 036 | string_basic | std::string | ✅ | `hicc_std::string` | `string::from(c"...")` |
| 037 | array_basic | std::array | ✅ | `hicc_std::array` | 同 034 |
| 038 | tuple_basic | std::tuple | ⚠️ 部分 | 需 cpp! 块写 field_N 访问器 | `static auto& field_0(tuple&);` |
| 039 | lambda_basic | lambda | ✅ | 转 `std::function` 后用 `hicc::Function<...>` | `Function<fn(i32) -> i32>` |
| 040 | std_function | std::function | ✅ | `hicc::Function<fn(...) -> R>` | 同 039 |
| 041 | functional_bind | std::bind | ✅ | cpp! 块包装成 `std::function` 后导出 | `static std::function<...> make(...);` |
| 042 | exception_basic | 异常 | ✅ | `hicc::Exception<T>` | `fn risky() -> Exception<i32>;` |
| 043 | namespace_nested | 嵌套命名空间 | ✅ | C++ 侧命名空间对 FFI 透明 | 完整类名 `n1::n2::Foo` |
| 044 | enum_class | enum class | ✅ | cpp! 块写 `int to_int(E)` 包装 | `static int to_int(E);` |
| 045 | union_basic | union | ⚠️ 部分 | cpp! 块写 variant accessor | POD union 可直接，非平凡需包装 |
| 046 | constexpr_basic | constexpr | ✅ | 等价于常量静态变量，`#[cpp(data)]` | `#[cpp(data = "X::PI")] fn pi() -> &'static f64;` |
| 047 | noexcept_basic | noexcept | ✅ 自动忽略 | C++ noexcept 不影响 FFI | 同 001 |
| 048 | summary | 综合示例 | ✅ | 多种模式混用 | 各模式组合 |

## 表 2：6 批分组（每批 ≤ 8 项，按主题分组）

| 批 | 范围 | 主题 | 共性 hicc 模式 | 预计难点 |
|---|---|---|---|---|
| A | 001-008 | 函数 + 类构造/拷贝 | `import_lib` / `import_class` 基础 | 工厂函数包装构造函数 |
| B | 009-016 | move/static/const + 继承基础 | 静态成员方法 + 继承语法 | volatile 的 Rust 表达 |
| C | 017-024 | virtual/diamond/operator/friend/explicit/mutable/typeid + 模板函数 | `#[interface]` + `cpp!` 包装（operator/typeid） | operator_overload 必须用 cpp! 包装 |
| D | 025-032 | 模板类 + 智能指针 | `template<...>` 签名 + hicc::unique_ptr | 自定义 deleter 类型映射 |
| E | 033-040 | RAII + STL + 函数式 | `hicc-std` + `hicc::Function` | lambda 转 std::function |
| F | 041-048 | functional 收尾 + 异常/namespace/enum/union/constexpr/noexcept | 异常 + 各种 cpp! 包装 | 047 允许 C++ 最小修改 |

## 表 3：按 hicc 模式分布统计

| hicc 模式 | 涉及特性数 | 特性编号 |
|---|---|---|
| `import_lib!`（普通函数） | 8 | 001, 002, 003, 004, 005, 047, 020, 021 |
| `import_class!`（普通类） | 14 | 006, 007, 008, 009, 010, 011, 013, 014, 022, 033, 043, 018, 027, 026 |
| `#[interface]`（抽象类/虚函数） | 4 | 015, 016, 017 |
| `hicc-std`（STL 容器） | 5 | 034, 035, 036, 037, 038 |
| `hicc::Function<...>`（函数式） | 3 | 039, 040, 041 |
| `hicc::Exception<T>`（异常） | 1 | 042 |
| `hicc::unique_ptr/shared_ptr`（智能指针） | 3 | 029, 030, 031 |
| `hicc::placement_new` | 1 | 032 |
| 模板类 / 模板函数 | 5 | 024, 025, 026, 027, 028 |
| `cpp!` 块包装（绕过） | 5 | 019 (operator), 023 (typeid), 038 (tuple), 044 (enum), 045 (union) |
| `#[cpp(data)]` 静态/常量 | 1 | 046 |
| ⚠️ 部分 / 特殊处理 | 2 | 012 (volatile), 047 (noexcept) |
| 综合 | 1 | 048 |

## Phase 2 → Phase 3 衔接

每个特性的 P3 实现按以下决策树选模板：

1. **如果是普通全局/静态函数** → 用 cpp2rust-demo 003 的 `cpp/` 三件套 + `rust_hicc/` 模板（见 hicc-capabilities.md §9）。
2. **如果是类** → cpp 端写 `extern "C"` 的工厂函数 + `class_*` 方法（C ABI 友好），rust_hicc 用 `import_class!` 包装。
3. **如果涉及 STL 容器** → cpp 端 `#include <hicc/std/{container}.hpp>` + typedef，rust_hicc 用 `hicc_std::{container}` 别名。
4. **如果涉及 operator/typeid/union/enum 包装** → cpp 端在 `extern "C"` 函数里写 accessor，rust_hicc 当普通函数导出。
5. **如果涉及智能指针自定义 deleter** → rust_hicc 显式用 `hicc::unique_ptr<T>` 而非裸 T。
6. **如果涉及虚函数 / 抽象类** → rust_hicc 用 `#[interface]` + 工厂函数 `@make_proxy<T>()`。
