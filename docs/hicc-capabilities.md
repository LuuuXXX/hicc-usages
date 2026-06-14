# hicc 能力地图

> 目的：在不修改 hicc 任何源码的前提下，搞清 9 个子项目各自提供什么、不提供什么。本文是 `cpp-feature-matrix` 方案的 P1 产物，后续 48 个示例的手动 AST→hicc 映射都基于本文的清单选模式。

## 1. 子项目一览

| 子项目 | 方向 | 角色 | 关键 API |
|---|---|---|---|
| `hicc` | C++ → Rust | 核心库：宏 + 运行时类型 + Trait | `cpp!` / `import_lib!` / `import_class!`、`AbiClass`、`ClassRef`、`ClassRefMut`、`ClassPtr`、`ClassMutPtr`、`Pod<T>`、`Exception<T>`、`Function<fn()>`、`Interface<T>`、`unique_ptr<T>`、`shared_ptr<T>` |
| `hicc-macros` | C++ → Rust | 过程宏入口（薄壳） | `import_lib` / `import_class` / `cpp` |
| `hicc-autogen` | 内部 | syn 解析器：把宏输入转成 Rust 代码 + cpp 适配代码 | （仅内部用，下游不直接调） |
| `hicc-build` | C++ → Rust | build.rs 驱动：扫 rust 文件、生成 cpp 适配代码、调 cc::Build 编静态库 | `Build::new().rust_file(...).compile(name)` |
| `hicc-std` | C++ → Rust | STL 容器的现成 Rust 包装 | `hicc_std::string / vector / array / deque / list / forward_list / set / unordered_set / map / unordered_map / stack / queue` |
| `hicc-rs` | Rust → C ABI | 反向：把 Rust 类型/函数导出为 C ABI | `#[export_class]` / `#[export_lib]` / `foreign!()` |
| `hicc-rs-macros` | Rust → C ABI | 过程宏入口 | （仅 hicc-rs 内部用） |
| `hicc-cbindgen` | Rust → C ABI | 工具：从 hicc-rs crate 生成 C 头 / Python / Cython 绑定 | CLI: `hicc-cbindgen -c <crate> -l c/python/cython -o <out>` |
| `hicc-examples` | 文档 | 官方教程样例 | `hello-world / import_lib_class / interface / dynamic_cast / destroy / functional / placement_new / datas / rust_any / memory / class / functions / stl` |

> 本仓库场景是 **C++ → Rust**，所以主要用 `hicc / hicc-macros / hicc-build / hicc-std`。`hicc-rs / hicc-rs-macros / hicc-cbindgen` 是反向链路（Rust 暴露给 C/Python），本文档暂不展开，仅在 README 矩阵中标注哪些 C++ 特性如果反向会用到。

## 2. 三大宏速查

### 2.1 `hicc::cpp! { ... }` — 内嵌 C++ 代码块
作用：把宏内的 C++ 源码原样喂给 hicc-build 生成的 cpp 文件。常见用法是 `#include` 头文件 + 写内联 C++ 适配函数。

```rust
hicc::cpp! {
    #include <iostream>
    #include "my_header.h"
    static void adapter() { /* C++ 实现 */ }
}
```

也可作为「灵活适配层」嵌套在 `import_class!` 内部，弥补 hicc 自动适配覆盖不到的接口（参见 reference.md 末尾示例）。

### 2.2 `hicc::import_lib!` — 声明 C++ 全局/静态函数

```rust
hicc::import_lib! {
    #![link_name = "example"]      // 必须，全局唯一，作为静态库名

    // 可选：在当前代码块内建立 C++ 类型别名
    class Foo;                       // Foo 代表 C++ 类 Foo
    class Bar = some::Bar;           // Bar 代表 C++ some::Bar
    class VecI32 = std::vector<int>; // 显式给容器起名

    // 普通 C++ 函数
    #[cpp(func = "int add(int, int)")]      // 注意：只写类型，不写形参名
    fn add(a: i32, b: i32) -> i32;

    // 模板函数（含模板参数列表）
    #[cpp(func = "std::unique_ptr<std::string> std::make_unique<std::string, const char*>(const char*&&)")]
    unsafe fn string_from(s: *const i8) -> hicc_std::string;

    // 把工厂函数附加到 Rust struct 的关联函数（生成 impl Foo）
    #[cpp(func = "Foo* Foo::new_instance()")]
    #[method(class = Foo, name = new)]
    fn foo_new() -> Foo;

    // 缺省参数：Rust 端省略尾部参数即可
    #[cpp(func = "int foo(int, int)")]
    fn foo(a: i32) -> i32;

    // 异常捕获：返回 hicc::Exception<T>
    #[cpp(func = "int risky(int)")]
    fn risky(v: i32) -> hicc::Exception<i32>;

    // 变长参数（最后参数是 va_list）
    #[cpp(func = "void log(const char*, va_list)")]
    unsafe fn log(fmt: *const i8, ...);
}
```

### 2.3 `hicc::import_class!` — 声明 C++ 类（成员方法 / 字段 / 继承）

```rust
hicc::import_class! {
    // 类型别名（同 import_lib）
    class Bar = bar::Bar;

    #[cpp(class = "Foo")]                // 必填：C++ 类完整签名
    class Foo {
        #[cpp(method = "void bar() const")]      // const 成员函数
        fn bar(&self);

        #[cpp(method = "void bump()")]           // 非 const 成员函数
        fn bump(&mut self);

        // 字段（值返回引用）
        #[cpp(field = "count")]
        fn count(&self) -> &usize;

        // 静态成员变量
        #[cpp(data = "Foo::g_count")]
        fn g_count() -> &'static usize;

        // 构造函数（必须放 import_lib，但可在 class 内做 Rust 包装）
        fn new(x: i32) -> Self {
            unsafe { foo_new(x) }
        }

        // 右值引用方法（self 转移所有权）
        #[cpp(method = "std::string into() &&")]
        fn into_name(self) -> hicc_std::string;
    }

    // 模板类
    #[cpp(class = "template<class T, class Alloc> std::vector<T, Alloc>")]
    pub class vector<T> {
        #[cpp(method = "bool is_empty() const")]
        pub fn is_empty(&self) -> bool;
    }

    // 接口（虚函数 / 抽象类）
    #[interface]
    class AnimalTrait {
        #[cpp(method = "void speak() const")]
        fn speak(&self);
    }

    // 继承
    #[cpp(class = "Baz", ctor = "Baz()")]   // ctor 指明默认构造
    class Baz: Bar {
        #[cpp(method = "void baz() const")]
        fn baz(&self);
    }

    // 私有析构：destroy = 释放函数名
    #[cpp(class = "Foo", destroy = "Foo::free_instance")]
    class Foo { /* ... */ }
}
```

### 2.4 `import_lib` / `import_class` 中的内置函数（builtin）

| builtin | 出现位置 | 作用 |
|---|---|---|
| `T @make_proxy<T>()` | import_lib 的 `#[cpp(func = ...)]` | 创建 Rust 实现的代理对象，参数必须是 `hicc::Interface<T>`，结合 `#[interface(name = ...)]` 一起用，实现 C++ 抽象类的 Rust 侧实现 |
| `T @dynamic_cast<T>()` | import_lib / import_class | C++ `dynamic_cast`，需要在 Rust 侧为每个目标类型单独定义接口 |
| `hicc::placement_new<T, Args...>(buf, len, args...)` | import_lib | 在 Rust 提供的内存上构造 C++ 对象，返回借用（生命周期绑定到输入缓冲） |

## 3. hicc-build API

`Build` 是 `cc::Build` 的包装（`Deref/DerefMut`），所以可以直接调 `cc::Build` 的方法（`include / file / cpp(true) / flag(...) / ...`）。

```rust
fn main() {
    let cpp_dir = std::path::PathBuf::from("../cpp");
    let mut build = hicc_build::Build::new();      // 自动 include hicc 头 + 设置 cpp(true)
    {
        let cc: &mut cc::Build = build.deref_mut();
        cc.include(&cpp_dir).include(".")
          .cpp(true)
          .file(cpp_dir.join("my_lib.cpp"));
    }
    build.rust_file("src/lib.rs")                  // 必填：扫这个文件的 hicc 宏
         .compile("my_lib");                       // 静态库名（与 link_name 对应）

    println!("cargo::rustc-link-lib=my_lib");
    #[cfg(not(all(target_os = "windows", target_env = "msvc")))]
    println!("cargo::rustc-link-lib=stdc++");
    println!("cargo::rerun-if-changed=src/lib.rs");
    println!("cargo::rerun-if-changed=../cpp/my_lib.cpp");
}
```

## 4. hicc-std 已封装的 STL 容器

`hicc-std/src/lib.rs` 导出以下模块：

| Rust 模块 | C++ 类型 | 备注 |
|---|---|---|
| `std_string` | `std::string / u16string / u32string` | 提供 `from(c"...")`、`as_cstr()`、`len()`、`insert`、`append` 等 |
| `std_vector` | `std::vector<T>` | 泛型，POD 用 `hicc::Pod<T>` |
| `std_array` | `std::array<T, N>` | N 在 C++ 侧 typedef 固定 |
| `std_deque` | `std::deque<T>` | |
| `std_list` | `std::list<T>` | |
| `std_forward_list` | `std::forward_list<T>` | |
| `std_set` | `std::set<T>` | |
| `std_unordered_set` | `std::unordered_set<T>` | |
| `std_map` | `std::map<K, V>` | |
| `std_unordered_map` | `std::unordered_map<K, V>` | |
| `std_stack` | `std::stack<T>` | |
| `std_queue` | `std::queue<T>` | |

> **使用约束**：模板参数必须是 C++ 类（即 `AbiClass`）或 `hicc::Pod<T>` 包裹的 POD。容器类型需在 `hicc::cpp!` 块中先 `typedef` 一个名字，再在 `import_lib` 里 `class Xxx = hicc_std::vector<hicc::Pod<i32>>;` 起别名 + 写一个 `make_unique<T>()` 工厂函数。

## 5. 类型映射核心规则（来自 `hicc/src/lib.rs` 顶部）

### 5.1 函数返回类型映射

| C++ 返回类型 | Rust 类型 |
|---|---|
| `T` | `T` |
| `T&&` | `T` |
| `std::unique_ptr<T>` | `T` |
| `std::unique_ptr<T, D>` | `hicc::unique_ptr<T>` |
| `std::shared_ptr<T, D>` | `hicc::shared_ptr<T>` |
| `const T&` | `hicc::ClassRef<'_, T>` |
| `T&` | `hicc::ClassRefMut<'_, T>` |
| `T*` | `hicc::ClassMutPtr<'_, T, 1>` |
| `const T*` | `hicc::ClassPtr<'_, T, 1>` |

> 多重指针 `T**` → `ClassPtr<'_, T, 2>`，`N` 表示指针重数。

### 5.2 函数参数类型映射

| C++ 参数类型 | Rust 参数 |
|---|---|
| `T` / `T&&` / `std::unique_ptr<T>` | `T` |
| `std::unique_ptr<T, D>` | `hicc::unique_ptr<T>` |
| `const T&` | `&T` |
| `T&` | `&mut T` |
| `const T*` | `&hicc::ClassPtr<'_, T, 1>` |
| `T*` | `&hicc::ClassMutPtr<'_, T, 1>` |

### 5.3 模板参数（泛型 T）类型映射

| C++ 模板参数 | Rust 输入 | Rust 输出 |
|---|---|---|
| `T` / `T&&` | `<T as AbiType>::InputType` | `<T as AbiType>::OutputType` |
| `const T&` | `&<T as AbiType>::InputType` | `<T as AbiType>::OutputRef<'_>` |
| `T&` | `&mut <T as AbiType>::InputType` | `<T as AbiType>::OutputRefMut<'_>` |
| `const T*` | `<T as AbiType>::InputPtr<'_>` | `<T as AbiType>::OutputPtr<'_>` |
| `T*` | `<T as AbiType>::InputMutPtr<'_>` | `<T as AbiType>::OutputMutPtr<'_>` |

### 5.4 自动 `&T → ClassRef<'_, T>` 转换

如果某个 `import_lib!` / `import_class!` 块里通过 `class Foo;` 或 `class Foo = ...;` 声明了 C++ 类型，那么该块内的 Rust 函数返回类型写 `&Foo` 会被宏自动改写为 `ClassRef<'_, Foo>`，写 `&mut Foo` 改为 `ClassRefMut<'_, Foo>`。这能消除"返回 C++ 引用"的内存安全风险。

## 6. Trait 与运行时类型

| Trait / 类型 | 作用 | 常用 API |
|---|---|---|
| `AbiClass` | 所有 C++ 类映射类型都实现 | `is_null() / write(&val) / make_unique() / make_ref() / make_ref_mut()` |
| `AbiType` | 所有可跨 ABI 的类型实现（含模板参数） | `Output / OutputRef / OutputRefMut / OutputPtr / OutputMutPtr / Input / InputType` |
| `ClassRef<'a, T>` | `const T&` 的对应 | deref 到 `&T` |
| `ClassRefMut<'a, T>` | `T&` | deref_mut 到 `&mut T` |
| `ClassPtr<'a, T, N>` | `const T*...*` | 多重只读指针 |
| `ClassMutPtr<'a, T, N>` | `T*...*` | 多重可写指针 |
| `Pod<T>` | POD 模板参数的标记 | `vector<hicc::Pod<i32>>` |
| `Exception<T>` | 捕获 C++ 异常 | `.ok() -> Result<T, String>` |
| `Function<fn(...) -> R>` | `std::function<R(Args...)>` | Rust 闭包 `.into()` 后传入；返回值 `.into()` 后调用 |
| `Interface<T>` | Rust 实现 C++ 接口的代理桥 | `@make_proxy<T>()` 的参数类型 |
| `unique_ptr<T>` | `std::unique_ptr<T, D>` 自定义 deleter | |
| `shared_ptr<T>` | `std::shared_ptr<T, D>` 自定义 deleter | |

## 7. 已知 hicc 直接覆盖的能力清单

| 能力 | hicc 提供 | 用法 |
|---|---|---|
| 全局 / 静态 / 模板函数 | ✅ | `import_lib!` + `#[cpp(func = ...)]` |
| 缺省参数 | ✅ | Rust 端省略尾部参数 |
| 忽略返回值 | ✅ | Rust 函数不写返回类型 |
| 异常 | ✅ | `hicc::Exception<T>` |
| 变长参数（C 风格 `...` 或 `va_list`） | ✅ | Rust `unsafe fn f(...)` |
| 类成员函数（const / 非 const / 右值） | ✅ | `import_class!` + `#[cpp(method = ...)]` |
| 类字段（成员变量） | ✅ | `#[cpp(field = ...)]` |
| 全局 / 静态变量 | ✅ | `#[cpp(data = ...)]` |
| 继承（单 / 多 / 虚继承） | ✅（语义级） | `class Derived: Base` 语法 |
| 抽象类 / 虚函数 / 纯虚 | ✅ | `#[interface]` + `@make_proxy<T>()` |
| `dynamic_cast` | ✅ | `@dynamic_cast<T>()` |
| 私有析构 | ✅ | `#[cpp(class = ..., destroy = ...)]` |
| `std::function` / 闭包 | ✅ | `hicc::Function<fn(...) -> R>` |
| 模板函数 / 模板类 | ✅ | 完整 `template<...>` 签名 |
| `unique_ptr` / `shared_ptr` | ✅ | 默认 deleter 自动映射；自定义 deleter 用 `hicc::unique_ptr<T>` |
| STL 容器 | ✅ | hicc-std 12 个容器 |
| placement new | ✅ | `hicc::placement_new<T, Args>(buf, len, args)` |
| C++ 容器存 Rust 数据 | ✅ | `hicc::RustAny<T>` + `RustKey` / `RustHashKey` |

## 8. 已知 hicc **不直接支持**或需绕过的能力

| 能力 | 状态 | 绕过方式 |
|---|---|---|
| 操作符重载（`operator+` 等） | ❌ 直接 | 在 `hicc::cpp!` 块写 `static T add(const T&, const T&) { return a + b; }` 包装函数，再 `import_lib!` 导出 |
| RTTI `typeid(T)` / `typeid(obj).name()` | ❌ 直接 | 包装成 `const char* hicc_typeid_name(const T&)` 在 cpp! 块返回 |
| `volatile` 修饰 | ⚠️ 部分 | C++ 侧可以保留 `volatile`，但 Rust 侧没有对应概念；用 `&mut T` + 文档说明 |
| `union`（带非平凡成员） | ❌ 直接 | 在 cpp! 块写 accessor 函数（get/set 各 variant） |
| `constexpr` 全局常量 | ✅ 同静态变量 | `#[cpp(data = "...")]` |
| `noexcept` | ✅ 自动忽略 | Rust 没有等价修饰，C++ 侧的 noexcept 不影响 FFI 签名 |
| 多返回值 / 结构化绑定 | ⚠️ | 用 struct 包；或用 `std::tuple` 经 hicc-std |
| C++17 结构化绑定 / 折叠表达式 / if-constexpr | ✅ | 这些是编译期特性，对 FFI 透明 |
| 协变返回类型 | ⚠️ | 在 Rust 侧手写转换 |

## 9. 单特性 Rust crate 模板（用于 Phase 5）

```text
examples/{NNN_name}/
├── cpp/
│   ├── {name}.h              # extern "C" 接口
│   ├── {name}.cpp            # 实现
│   ├── main.cpp              # 独立可执行入口
│   ├── standalone.sh         # g++ 一键构建+运行
│   ├── Makefile
│   └── CMakeLists.txt
├── ast/
│   ├── {name}.i              # 宏展开后源（.gitignore）
│   └── ast.json              # clang JSON AST（.gitignore）
├── rust_hicc/
│   ├── Cargo.toml
│   ├── build.rs              # 驱动 cc::Build + hicc_build::Build
│   ├── src/lib.rs            # hicc::cpp! + import_lib!/import_class!
│   └── tests/smoke.rs
└── ast-to-hicc-notes.md      # 手动 AST→hicc 方案记录
```

`rust_hicc/Cargo.toml`（关键依赖用相对路径指向仓库内的 hicc）：

```toml
[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[lib]
name = "{name}"
path = "src/lib.rs"

[[bin]]
name = "{name}"
path = "src/main.rs"

[dependencies]
hicc = { path = "../../../hicc/hicc", version = "0.2" }
hicc-std = { path = "../../../hicc/hicc-std", version = "0.2" }  # 按需

[build-dependencies]
cc = "1.0"
hicc-build = { path = "../../../hicc/hicc-build", version = "0.2" }
```

`rust_hicc/build.rs`（标准模板）：

```rust
fn main() {
    let cpp_dir = std::path::PathBuf::from("../cpp");
    let mut build = hicc_build::Build::new();
    use std::ops::DerefMut;
    let cc_build: &mut cc::Build = build.deref_mut();
    cc_build.include(&cpp_dir).include(".").cpp(true)
            .file(cpp_dir.join("{name}.cpp"));

    build.rust_file("src/lib.rs").compile("{name}");

    println!("cargo::rustc-link-lib={name}");
    #[cfg(not(all(target_os = "windows", target_env = "msvc")))]
    println!("cargo::rustc-link-lib=stdc++");
    println!("cargo::rerun-if-changed=src/lib.rs");
    println!("cargo::rerun-if-changed=../cpp/{name}.cpp");
    println!("cargo::rerun-if-changed=../cpp/{name}.h");
}
```

## 10. 已知约束（来自 hicc 源码 / reference.md）

1. **依赖 C++11 或更高**。
2. **不直接支持操作符重载**，需 `cpp!` 块包装。
3. **`build.rs` 中 `rust_file()` 扫描整个 .rs 文件**，宏以外的 item 会被忽略，但文件必须能被 syn 解析。
4. **`hicc-build` 只产静态库**，最终二进制要手动 `println!("cargo::rustc-link-lib=stdc++")` 链接 C++ 标准库。
5. **`import_lib!` / `import_class!` 的 `#[cpp(func/method/...)]` 声明只写类型，不写形参名**。这是常见的踩坑点。
6. **模板参数** 只能是 C++ 类或 `hicc::Pod<T>` 包裹的 POD。
7. **容器类型**必须先在 `cpp!` 块 `typedef`，再在 `import_lib` 起别名 + 工厂函数。
