# hicc 能力图谱

> 子模块 `hicc/` 是**只读**的 — 本项目从不修改它的源码（除 `examples/047_noexcept_basic` 的 C++ 端做了一处 `noexcept` 移除，已在该特性 README 醒目标注）。

## 1. 9 个子 crate 的职责

| 子 crate | 类型 | 职责 | 本项目是否使用 |
|----------|------|------|----------------|
| `hicc` | lib（核心） | 安全 FFI 框架本体：`hicc::cpp!`、`import_lib!`、`import_class!` 宏、`Exception<T>`、`Pod<T>`、`make_unique` | ✅ 每个特性都用 |
| `hicc-macros` | proc-macro | 解析 `import_lib!` / `import_class!` 的 Rust 端语法树 | 间接（hicc 依赖） |
| `hicc-build` | build-dep | 在 `build.rs` 里读 Rust 源码，生成 C++ 适配代码 + 调 `cc::Build` 编译 | ✅ 每个特性的 `rust_hicc/build.rs` |
| `hicc-autogen` | lib | `hicc-build` 内部的代码生成器（`ExportLib` / `ExportClasses` / `Cpp`） | 间接 |
| `hicc-std` | lib | 把 C++ 标准库容器（vector / map / array / string 等）映射为 Rust 类型 | ✅ 034-037 STL 特性 |
| `hicc-rs` | lib | 反向 FFI：Rust → C ABI | ❌（本项目只走 C++ → Rust 方向） |
| `hicc-rs-macros` | proc-macro | `hicc-rs` 的过程宏 | ❌ |
| `hicc-cbindgen` | build-dep | 生成 `hicc-rs` 的 C 头文件 | ❌ |
| `hicc-examples` / `hicc-rs-examples` | examples | 官方示例（C++ 内联在 Rust 源码中） | 参照用 |

### 依赖图

```
                ┌──────────────────────────────────────┐
                │ hicc-macros (proc-macro)             │
                │   解析 import_lib!/import_class! 语法 │
                └────────────┬─────────────────────────┘
                             │
              ┌──────────────▼──────────────┐
              │ hicc (core lib)             │
              │   Exception<T> / Pod<T> /   │
              │   make_unique / cpp! / ...  │
              └────┬────────────────┬───────┘
                   │                │
       ┌───────────▼────┐     ┌─────▼─────────────────────┐
       │ hicc-std       │     │ hicc-build (build-dep)    │
       │  STL 容器适配   │     │  读 Rust 源 → 生成 C++ 适配 │
       └────────────────┘     │  → cc::Build 编译          │
                              └────┬──────────────────────┘
                                   │
                            ┌──────▼──────────────┐
                            │ hicc-autogen        │
                            │  ExportLib/Classes  │
                            └─────────────────────┘

  (hicc-rs / hicc-rs-macros / hicc-cbindgen：反向 FFI，本项目不用)
```

## 2. hicc 的核心使用模式

### 模式 A：内联 C++（hicc 官方示例风格）

C++ 代码放在 Rust 源码的 `hicc::cpp! { ... }` 块内，hicc-build 自动抽取并编译。

```rust
hicc::cpp! {
    #include <iostream>
    static void hello_world() { std::cout << "hello"; }
}

hicc::import_lib! {
    #![link_name = "example"]
    #[cpp(func = "void hello_world()")]
    fn hello_world();
}
```

### 模式 B：外部 C++（本项目使用 ✅）

C++ 项目独立放在 `cpp/`，预先用 standalone/Make/CMake 构建出 `lib<name>.a`。Rust 端 `hicc::cpp!` 块**只**放 `#include` 把外部头文件带入，绑定靠 `import_lib!` / `import_class!`，`build.rs` 用 `cargo::rustc-link-lib` 链接外部库。

```rust
// rust_hicc/src/lib.rs
hicc::cpp! {
    #include "hello_world.h"     // 外部头文件
}

hicc::import_lib! {
    #![link_name = "<name>_hicc"]   // hicc 生成的 adapter 库名
    #[cpp(func = "void hello_world()")]
    fn hello();
}
```

```rust
// rust_hicc/build.rs
hicc_build::Build::new()
    .rust_file("src/lib.rs")
    .compile("<name>_hicc");              // 编译 hicc adapter

println!("cargo::rustc-link-search=native=../cpp/build");
println!("cargo::rustc-link-lib=<name>"); // 外部 C++ 实现
println!("cargo::rustc-link-lib=stdc++");
```

**模式 B 的关键约束**：
- `hicc::cpp!` 块只放 `#include`，**不要**重复 C++ 实现，否则与外部 `.a` 冲突
- `import_class!` / `import_lib!` 中 `#[cpp(method = "ret f(args)")]` 的签名必须与 `cpp/<name>.h` 中的声明完全一致
- `link_name` 必须唯一，避免与外部库名冲突（约定加 `_hicc` 后缀）

## 3. C++ 构造 → hicc 支持表

| C++ 构造 | hicc 支持度 | 通过什么支持 | 关键代码 |
|----------|-------------|--------------|----------|
| 自由函数 | ✅ 直接 | `import_lib!` + `#[cpp(func=...)]` | `fn hello();` |
| 函数重载 | ✅ 直接 | Rust 端加类型后缀 | `fn add_i32(...)` `fn add_f64(...)` |
| 默认参数 | ✅ 直接 | Rust 端补完整签名 | `#[cpp(func = "int f(int, int)")]` |
| inline 函数 | ✅ 直接 | 透明 | 同普通函数 |
| variadic `...` | ⚠️ C++ 端调整 | C++ 写固定 arity 包装 | `int sum_n(int n, ...)` → `int sum2(int,int)` |
| 类（基础） | ✅ 直接 | `import_class!` + factory | `class Foo { fn foo(&self...); }` |
| 构造/析构 | ✅ 直接 | factory + `destroy=` | `fn foo_new() -> Foo;` |
| 拷贝构造 | ✅ 直接 | C++ 写 `T clone(T*)` 包装 | 命名函数 |
| 移动构造 | ✅ 直接 | `T&&` 方法用 `self` 接收 | `fn take(self, ...) -> Self;` |
| 静态成员 | ✅ 直接 | 在 `import_lib!` 中描述 | `#[cpp(func = "int Foo::count()")]` |
| const 成员 | ✅ 直接 | `#[cpp(method = "ret f() const")]` | `fn foo(&self, ...) -> ...` |
| volatile 成员 | ✅ 直接 | `#[cpp(method = "ret f() volatile")]` | `fn foo(&mut self, ...)` |
| 单继承 | ✅ 直接 | 派生类独立 `import_class!` | 合并基类公共方法 |
| 多继承 | ✅ 直接（或 `#[interface]`） | 同上 或 `@make_proxy` | — |
| 虚函数 / 纯虚 / override | ✅ 直接 | 普通方法路径（vtable 透明） | — |
| 菱形虚继承 | ⚠️ 限制标注 | 简化为组合 | README 标注 |
| 运算符重载 | 💬 注释式注入 | C++ 写命名包装函数 | `vec2_add` |
| 友元函数 | ✅ 直接 | 暴露为自由函数 | — |
| explicit 构造 | ✅ 直接 | factory 模式 | — |
| mutable 成员 | ✅ 直接 | 透明（const 方法可改） | — |
| typeid / RTTI | ✅ 直接 | 命名函数返回 type info | `type_name_of(T*) -> string` |
| 函数模板 | ✅ 直接 | `#[cpp(func = "ret f<T>(args)")]` | 显式实例化 |
| 类模板 | 💬 活跃注入 | `hicc::cpp!` 内 `using FooInt = Foo<int>;` + factory | typedef + factory |
| 模板特化 | 💬 活跃注入 | namespace 级包装调用特化静态方法 | — |
| 显式实例化 | 💬 活跃注入 | typedef + factory | — |
| 变参模板 | 💬 活跃注入 | 固定 arity 包装（sum_two / sum_three） | — |
| unique_ptr | ✅ 直接 | 返回值类型剥 `unique_ptr<>` | `fn make() -> Foo;` |
| shared_ptr | ✅ 直接 | 同上 | — |
| 自定义删除器 | ✅ 直接 | `destroy="free_func"` | `import_class! { ... destroy="..."; }` |
| placement new | ✅ 直接 | factory `T* construct_at(buf, args)` | — |
| RAII | ✅ 直接 | `destroy=` 给 Drop | — |
| std::vector | ✅ 直接（hicc-std） | `class Vec = hicc_std::vector<Pod<T>>` | — |
| std::map / unordered_map | ✅ 直接（hicc-std） | 同上 | — |
| std::string | ✅ 直接 | **必须** `import_class! class string` | **不可**用 `hicc_std::string` |
| std::array | ✅ 直接（hicc-std） | typedef `CppArr=std::array<T,N>` | — |
| std::tuple | ⚠️ 限制标注 | 命名 accessor first/second | — |
| lambda | ⚠️ C++ 端调整 | C++ 写命名包装函数 | — |
| std::function | ⚠️ C++ 端调整 | 同上 | — |
| std::bind | ⚠️ 限制标注 | 命名包装 | — |
| throw 异常 | ✅ 直接 | Rust 返回 `hicc::Exception<T>` | `.ok()` 转 Result |
| namespace（嵌套） | ✅ 直接 | Rust 短名 + `#[cpp]` 保留完整限定 | — |
| enum class | ⚠️ C++ 端调整 | int 转换函数 | `color_to_int/int_to_color` |
| union | 💬 注释式注入 | ValueBox 包装类（type_tag + from_X/as_X） | — |
| constexpr | ✅ 直接 | 透明 | — |
| noexcept 成员方法 | ⚠️ **C++ 端调整** | **唯一例外**：移除成员方法 noexcept（自由函数可保留） | — |
| 函数指针参数 | ❌ 不支持 | — | 改用 C++ 端包装 |
| 嵌套模板 `vector<vector<int>>` | ❌ 不支持 | — | 改用扁平结构 |

## 4. 限制清单（详细说明）

### 4.1 `noexcept` 成员方法（⚠️ 047 唯一例外）

**现象**：hicc-build 在生成 adapter 时，会对成员方法做类型匹配。如果 C++ 端是 `int foo() noexcept`，但 `import_class!` 中 `#[cpp(method = "int foo()")]`，签名不匹配，构建失败。

**降级**：
- **自由函数**：可保留 noexcept，Rust 端 `#[cpp(func = "void f() noexcept")]` 也可（hicc-build 透明处理）
- **成员方法**：C++ 端移除 `noexcept`（仅 047 这么做）

### 4.2 运算符重载（💬 注释式注入）

**现象**：hicc 无法直接绑定 `operator+` 等运算符（语法上不支持）。

**降级**：C++ 端写命名包装函数：
```cpp
Vec2 operator+(Vec2, Vec2);   // 原签名
Vec2 vec2_add(Vec2 a, Vec2 b) { return a + b; }  // 包装
```
Rust 端：`#[cpp(func = "Vec2 vec2_add(Vec2, Vec2)")]`

### 4.3 union（💬 注释式注入）

**现象**：Rust 没有 C++ 那种 union 概念，hicc 也没有直接支持。

**降级**：C++ 端写 ValueBox 包装类：
```cpp
union U { int i; float f; };
class UBox {
    int type_tag;
    union U data;
public:
    static UBox from_int(int);
    static UBox from_float(float);
    int as_int() const;
    float as_float() const;
};
```

### 4.4 lambda / std::function / std::bind（⚠️ C++ 端调整）

**现象**：闭包类型不可命名，跨 FFI 无法描述。

**降级**：C++ 端写命名包装函数。

### 4.5 菱形虚继承 / 嵌套模板（⚠️ 限制标注）

**现象**：vtable + offset 复杂；嵌套模板类型描述不出来。

**降级**：简化为组合 / 扁平结构，README 标注。

### 4.6 `std::string`（⚠️ Key Pattern）

**陷阱**：`hicc-std` 提供了 `hicc_std::string` alias，但**不要**用它绑定返回 std::string 的 C++ 函数。原因：hicc-std 的 string 是另一套类型层级，与 C++ 的 `std::string` 内存布局不兼容，跨 FFI 会段错误。

**正确做法**：
```rust
hicc::import_class! {
    #[cpp(class = "std::string")]
    class string {
        #[cpp(method = "const char* c_str() const")]
        fn c_str(&self) -> *const u8;
    }
}
```

### 4.7 类模板（💬 活跃注入）

**现象**：`Foo<int>` 不能直接作为 Rust 类型描述。

**降级**：在 `hicc::cpp!` 块内 typedef + factory：
```rust
hicc::cpp! {
    #include "foo.h"
    using FooInt = Foo<int>;
    inline FooInt* foo_int_new(int v) { return new Foo<int>(v); }
}

hicc::import_class! {
    #[cpp(class = "FooInt")]
    class FooInt {
        // ... methods ...
    }
}

hicc::import_lib! {
    #[cpp(func = "FooInt* foo_int_new(int)")]
    fn foo_int_new(v: i32) -> FooInt;
}
```

## 5. hicc-build 的 build.rs 约定

```rust
fn main() {
    // 1. 解析 lib.rs 中的 import_lib!/import_class!/cpp! 宏，生成 C++ 适配代码并编译
    hicc_build::Build::new()
        .rust_file("src/lib.rs")
        .compile("<lib_name>_hicc");

    // 2. 链接外部 C++ 静态库（项目模式 B）
    println!("cargo::rustc-link-search=native=../cpp/build");
    println!("cargo::rustc-link-lib=<name>");
    println!("cargo::rustc-link-lib=stdc++");

    // 3. 让生成的 adapter 找到我们的 C++ 头文件
    println!("cargo::rustc-flags=-I../cpp");

    // 4. 依赖追踪
    println!("cargo::rerun-if-changed=src/lib.rs");
    println!("cargo::rerun-if-changed=../cpp/build/lib<name>.a");
}
```

## 6. hicc-std 速查

```rust
use hicc_std::{Pod, vector, map, array};

hicc::import_class! {
    #[cpp(class = "std::vector<int>")]
    class Vec = vector<Pod<i32>>;          // 整数 vector

    #[cpp(class = "std::map<std::string,int>")]
    class Map = map<Pod<*const u8>, Pod<i32>>;  // 简化示意

    #[cpp(class = "std::array<int,3>")]
    class Arr = array<Pod<i32>, 3>;        // 定长数组
}
```

注意：`Pod<T>` 是 hicc-std 的"平凡可拷贝"包装。`string` 不要用 `hicc_std::string`（见 §4.6）。
