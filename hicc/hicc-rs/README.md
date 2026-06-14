# hicc-rs

将 Rust 类型和函数封装为 C ABI 的框架。通过过程宏自动生成 `#[repr(C)]` 的 FFI 类型和方法表，配合 `hicc-cbindgen` 生成 C 头文件或 Python ctypes 绑定。

## 核心概念

### AbiClass\<T\>

每个跨语言传递的 Rust 类型在 C 侧对应一个 `AbiClass<T>` 结构体：

```c
typedef struct AbiClass_Container_i32 {
    const struct AbiMethods_Container_i32 *methods;  // 虚方法表
    const void *this_;                                // 内部数据指针
    uintptr_t level;                                  // 指针重数（嵌套层数）
} AbiClass_Container_i32;
```

- 所有跨语言传递的值都是**堆分配**（`Box<T>`）
- `destroy` 隐式调用 Rust 的 `Drop` 释放堆内存

### 所有权模型

`AbiClass` 区分三种状态：

| 状态 | 含义 |
|------|------|
| **Value**（所有者） | 拥有堆内存，`destroy` 释放 |
| **Ref**（只读借用） | 借用，不可写入 |
| **RefMut**（可写借用） | 借用，可写入 |

**安全规则：**
- 返回的借用不能传递给需要所有权的接口（否则 panic）
- 返回的只读借用不能传递给需要可写借用的接口（否则 panic）

### 引用类型与 `'static` 生命周期

Rust API 中所有引用类型的参数和返回值在映射到 CABI 时，**必须使用 `'static` 生命周期**：

- 方法参数中的引用类型（如 `&T`、`&mut T`、`&str`、`&[T]`）在 CABI 层面全部替换为 `'static`
- 方法返回值中的引用类型同样替换为 `'static`
- impl self-type 中的生命周期（如 `Iter<'_, K, V>`、`MutexGuard<'_, T>`）宏自动替换为 `'static`

这是因为 C ABI 不具备 Rust 的生命周期概念，跨语言传递的引用必须独立于任何 Rust 局部借用栈。用户编写 API 时可使用任意生命周期标注（如 `'_`），宏会自动替换为 `'static`。

### 方法表

`AbiClass<T>` 的内置方法：

| 方法 | 说明 |
|------|------|
| `hicc_destroy` | 释放堆内存 |
| `hicc_make_unique` | 将借用提升为所有者 |
| `hicc_make_ref` | 转为只读借用 |
| `hicc_make_ref_mut` | 转为可写借用 |
| `hicc_size_of` | 返回 `T` 的大小 |
| `hicc_write` | 写入值（仅可写借用可用） |

加上用户通过 `#[export_class]` 声明的自定义方法。

## 过程宏

### `#[export_class]` — 暴露自定义类型的方法

将 `impl` 块中的方法生成为 C ABI 虚方法表，使 Rust 类型可跨语言调用。

**属性参数：**

| 参数 | 说明 |
|------|------|
| `foreign` | 封装第三方类型（见下文 [Foreign 模式](#foreign-模式---封装第三方类型)） |
| `in_hicc` | hicc-rs 内部使用 |

**用法：**

```rust
#![feature(specialization)]

use hicc_rs::export_class;

pub struct Container<T>(pub T);

// 方法声明（无函数体 → 宏自动生成转发代码）
#[export_class]
impl<T> Container<T> {
    fn get(&self) -> &T;
}

// 完整实现（有函数体 → 使用自定义逻辑）
#[export_class]
impl<T> Option<T> {
    fn as_ref(&self) -> *const T {
        self.as_ref().map(|v| v as *const T).unwrap_or(std::ptr::null())
    }
}
```

**mod 块 — 批量导出多个类型：**

多个 `impl` 块可以放在同一个 `mod` 块中，用同一个 `#[export_class]` 属性批量导出：

```rust
#[export_class(foreign)]
mod classes {
    impl<T> Vec<T> {
        fn push(&mut self, val: T);
        fn len(&self) -> usize;
    }

    impl String {
        fn len(&self) -> usize;
        fn as_str(&self) -> &str;
    }
}
```

mod 块中的每个 `impl` 块独立生成方法表和 trait 实现，非 `impl` 项（如 `use` 语句）原样保留。

**方法声明 vs 完整实现：**

- **声明**（无函数体，末尾 `;`）：宏自动生成转发代码 `obj.method(args)`
- **完整实现**（有函数体 `{ ... }`）：使用自定义逻辑，宏仅负责 ABI 转换

**异步方法：**

`export_class` 支持异步方法。异步方法的返回值类型在 CABI 层面映射为 `Box<dyn Future<Output = R>>`，通过 `HiccRuntime` 的 `wait` 或 `async_wait` 获取结果：

- **声明式**（末尾 `;`）：宏自动调用 `obj.method(args)` 得到 `Future`，再 `Box::new(obj.method(args))`
- **内联实现**（有函数体 `{ ... }`）：宏将 body 包裹为 `Box::new(async move { body })`

```rust
#[export_class]
impl AsyncCounter {
    // 声明式异步方法
    async fn async_increment(&self, delta: i32) -> i32;

    // 内联实现异步方法
    async fn async_greet(&self) -> String {
        format!("hello from {}", self.base)
    }
}
```

C/Python 侧调用异步方法时，返回 `AbiClass_Box_dynFuture_Output_R`，通过 `wait(runtime)` 阻塞等待结果，或 `async_wait(runtime, notify)` 异步回调。详见 [异步函数](#异步函数---futures)。

**约束：export_class 不支持无 self 接收者的方法**

`#[export_class]` 仅支持带有 `self` 接收者的方法（关联方法）。如果方法没有 `self`（即关联函数），宏会报错提示使用 `#[export_lib]`：

```rust
// ✗ 错误 — export_class 不支持关联函数
#[export_class]
impl Foo {
    fn new() -> Self;   // 编译错误：no self receiver
}

// ✓ 正确 — 关联函数应使用 export_lib
#[export_lib(name = "my_lib")]
mod ffi {
    fn new_foo() -> Foo { Foo::new() }
}
```

**约束：自引用泛型会导致无限单例化**

Rust 的泛型单例化机制要求每个具体类型组合独立编译。当方法返回值包含对自身泛型参数的引用层级时，会触发无限递归单例化：

```rust
// ✗ 错误：触发无限递归
#[export_class]
impl<T> Foo<T> {
    fn as_ref(&self) -> Foo<&T>;
}
// 单例化链：Foo<T> → Foo<&T> → Foo<&&T> → Foo<&&&T> → ...
```

避免方式：将返回类型中的引用层级截断，使用不依赖自身泛型的类型：

```rust
// ✓ 正确：返回裸引用或指针
#[export_class]
impl<T> Foo<T> {
    fn as_ref(&self) -> &T;
}
```

**引用/切片/指针/迭代器类型作为 self-type**

`export_class` 支持将 `&[u8]`、`&mut [T]`、`*const T`、`*mut T`、`Iter<'_, K, V>` 等引用、切片、指针、迭代器类型作为 impl 的 self-type。宏会自动将所有生命周期替换为 `'static`：

```rust
// 迭代器：使用 '_ 而非 'static，宏自动替换
use std::collections::hash_map::{Iter, IterMut};

#[export_class(foreign)]
impl<K, V> Iter<'_, K, V> {
    fn next(&mut self) -> Option<(&K, &V);
}

// 引用切片类型
#[export_class(foreign)]
impl &[u8] {
    fn len(&self) -> usize;
}

// 可变引用切片
#[export_class(foreign)]
impl &mut [i32] {
    fn len(&self) -> usize;
    fn set(&mut self, idx: usize, val: i32);
}

// 可变引用字符串
#[export_class(foreign)]
impl &mut str {
    fn len(&self) -> usize;
    fn get(&self, idx: usize) -> u8;
}
```

**注意：** 由于 Rust 的孤儿原则，在外部 crate 中上述类型必须使用 `foreign` 模式。在 hicc-rs 内部（`in_hicc`）可直接使用。

**where 子句中的 ValueType bound**

泛型参数的 `ValueType` bound 可以出现在 where 子句中，宏会正确识别：

```rust
#[export_class]
impl<T, U> WhereStruct<T, U>
where
    T: ValueType,
    U: ValueType,
{
    fn count(&self) -> i32;
}
```

宏为所有泛型参数自动追加 `ValueType + 'static` bound（用户显式写的 bound 也会保留），确保类型满足 CABI 的 `'static` 生命周期要求。

### `#[export_lib]` — 导出函数表

将 `mod` 块中的函数生成为 C ABI 函数指针结构体，提供一个 `extern "C"` 入口函数返回该结构体的静态引用。

**属性参数：**

| 参数 | 必需 | 说明 |
|------|------|------|
| `name = "xxx"` | 否 | 入口函数名，默认 `"hicc_export_lib"` |
| `foreign` | 否 | 函数中涉及第三方类型（见下文 [Foreign 模式](#foreign-模式---封装第三方类型)） |
| `in_hicc` | 否 | hicc-rs 内部使用 |

**用法：**

```rust
#![feature(specialization)]

use hicc_rs::export_lib;

// crate 根：函数实现
fn add(x: i32, y: i32) -> i32 { x + y }

pub struct Container<T>(pub T);

fn container_value(x: Container<i32>) -> i32 {
    *x.get()
}

fn new_container(x: i32) -> Container<i32> {
    Container(x)
}

// mod 块：声明 + use 引入
#[export_lib(name = "demo")]
mod lib {
    use super::*;
    fn add(x: i32, y: i32) -> i32;
    fn container_value(x: Container<i32>) -> i32;
    fn new_container(x: i32) -> Container<i32>;
}
```

**函数声明 vs 完整实现：**

- **声明**（无函数体，末尾 `;`）：宏自动调用 `函数名(args)` 转发到同 crate 的同名函数
- **完整实现**（有函数体 `{ ... }`）：使用自定义逻辑，宏仅负责 ABI 转换

**函数声明模式（推荐）：**

将函数实现移到 crate 根，mod 块内仅保留声明（末尾 `;`），通过 `use` 语句引入函数：

```rust
// crate 根：函数实现
fn add(x: i32, y: i32) -> i32 { x + y }
fn negate(x: i32) -> i32 { -x }

// mod 块：仅声明 + use 引入
#[export_lib(name = "demo")]
mod lib {
    use super::*;
    fn add(x: i32, y: i32) -> i32;
    fn negate(x: i32) -> i32;
}
```

声明模式下宏生成的代码使用简单函数名调用（不带 `crate::` 前缀），因此需要 `use` 语句将函数引入当前模块作用域。

也可以在同一 mod 块中混用声明和完整实现：

```rust
#[export_lib(name = "demo")]
mod lib {
    use super::*;
    fn add(x: i32, y: i32) -> i32;          // 声明 → 调用 crate 根的 add
    fn negate(x: i32) -> i32 { -x }          // 完整实现 → 使用自定义逻辑
}
```

**注意事项：**

- `export_lib` 中的函数**不能有泛型**，所有类型参数必须是具体类型
- `export_lib` 中的函数**不能有 `self` 接收者**（方法应使用 `#[export_class]`）
- `export_class` 和 `export_lib` 中的所有类型参数自动追加 `ValueType + 'static` 约束
- 需要导出关联函数时，应手工封装为普通函数：

```rust
#[export_lib(foreign, name = "my_lib")]
mod ffi {
    // 手工封装 String::new() 为普通函数
    fn new_string() -> String {
        String::new()
    }

    // 手工封装 Vec::<String>::new() 为普通函数
    fn new_vec_string() -> Vec<String> {
        Vec::new()
    }
}
```

### Foreign 模式 — 封装第三方类型

`foreign` 属性用于将**不属于当前 crate 的类型**（如 `Vec<T>`、`String`、`&[u8]`）封装为 C ABI 类型。由于 Rust 的**孤儿原则**（Orphan Rule）——只能在定义类型或定义 trait 的 crate 中为类型实现 trait——当前 crate 无法为第三方类型直接实现 hicc-rs 的 `ValueType`/`AbiType` 等 trait。`foreign` 模式通过 `ForeignType<T>` 代理层绕过此约束。

**启用：** 在 `src/lib.rs` 中调用 `hicc_rs::foreign!()` 宏：

```rust
#![feature(specialization)]
hicc_rs::foreign!();
```

**export_class + foreign：**

```rust
#[export_class(foreign)]
impl<T> Vec<T> {
    fn push(&mut self, val: T);
    fn len(&self) -> usize;
}
```

**export_lib + foreign：**

```rust
fn push(v: &mut Vec<String>, s: &str) -> String {
    v.push(s.to_string());
    s.to_string()
}

fn new_vec_string() -> Vec<String> {
    Vec::new()
}

#[export_lib(foreign, name = "foreign_type")]
mod ffi {
    use super::*;
    fn push(v: &mut Vec<String>, s: &str) -> String;
    fn new_vec_string() -> Vec<String>;
}
```

**Foreign\<T\> 自动包装规则：**

`foreign` 模式自动将生成的函数参数和返回值包装为 `Foreign<T>`。但如果需要在已经支持 CABI 转换的类型的泛型参数上使用 `Foreign<T>`，必须用户显式声明，例如 `HashSet<crate::hicc::Foreign<String>>`。这种模式下用户必须保证 `Foreign<T>` 中的 `T` 一定利用 `#[export_class(foreign)]` 支持转换为 CABI 类型。

```rust
// ✗ 错误 — 泛型参数位置不会自动包装，直接写 String 无法跨 FFI
fn process(s: HashSet<String>) -> HashSet<String>;

// ✓ 正确 — 泛型参数位置需显式写 Foreign<String>，且 String 已通过 export_class(foreign) 支持转换
fn process(s: HashSet<crate::hicc::Foreign<String>>) -> HashSet<crate::hicc::Foreign<String>>;
```

**Foreign\<T\> 内置 trait 实现：**

`Foreign<T>` 自动委托内层 `T` 的标准 trait 实现，使其可直接用于需要这些 trait 的场景（如 `HashSet<Foreign<String>>` 要求 `Hash + Eq`）：

| Trait | 条件 | 说明 |
|-------|------|------|
| `Deref` / `DerefMut` | 无 | 委托 `&self.0` / `&mut self.0`，可直接访问内层方法 |
| `Hash` | `T: Hash` | 委托 `self.0.hash(state)` |
| `PartialEq` / `Eq` | `T: PartialEq` / `T: Eq` | 委托 `self.0.eq(&other.0)` |
| `Clone` | `T: Clone` | 委托 `Foreign(self.0.clone())` |
| `Debug` | `T: fmt::Debug` | 委托 `self.0.fmt(f)` |
| `Display` | `T: fmt::Display` | 委托 `self.0.fmt(f)` |
| `PartialOrd` | `T: PartialOrd` | 委托 `self.0.partial_cmp(&other.0)` |
| `Ord` | `T: Ord` | 委托 `self.0.cmp(&other.0)` |

例如，`HashSet<Foreign<String>>` 可直接使用，因为 `String` 满足 `Hash + Eq`，`Foreign<String>` 自动继承：

```rust
use std::collections::HashSet;
use crate::hicc::Foreign;

let mut set: HashSet<Foreign<String>> = HashSet::new();
set.insert(Foreign("hello".to_string()));
assert!(set.contains(&Foreign("hello".to_string())));
```

## 生成 C / Cython / Python 绑定

配合 `hicc-cbindgen`：

```bash
# C 头文件（默认）
hicc-cbindgen -c hicc-rs-examples/basic_lib -o hicc_demo.h

# Cython .pxd 文件（同时生成 companion C 头文件供编译器使用）
hicc-cbindgen -c hicc-rs-examples/basic_lib -l cython -o hicc_demo.pxd

# Python ctypes 绑定（--lib 可省略，自动从 Cargo.toml 推导库名）
hicc-cbindgen -c hicc-rs-examples/basic_lib -l python -o hicc_demo.py
```

### ⚠ Python 绑定：destroy() 必须显式调用

Python 的 `__del__` 调用时机由 GC 决定，不可靠且不安全：

- GC 可能延迟回收，导致 Rust 堆内存长期不释放
- GC 在进程退出时可能跳过 `__del__`，导致内存泄漏
- 在引用循环、异常等场景下 `__del__` 可能根本不被调用

**因此，生成的 Python 包装器类不依赖 `__del__` 进行析构。用户必须在使用完毕后显式调用 `destroy()`：**

```python
# ✗ 错误 — 依赖 GC 自动回收，可能泄漏 Rust 内存
s = new_str()
result = s.len()

# ✓ 正确 — 显式调用 destroy()
s = new_str()
result = s.len()
s.destroy()
```

**规则**：
- 值传递 `self` 的方法（如 `unwrap`, `into_inner`）会消费 `self`，不需要再调用 `destroy()`
- 借用 `self` 的方法（如 `len`, `get`, `push`）不消费 `self`，**使用完毕后必须调用 `destroy()`**
- 返回新 AbiClass 的方法/函数，返回的对象也必须调用 `destroy()`

### Python 绑定的所有权注释

生成的 Python 绑定中，每个方法和工厂函数**始终包含** `"""Ownership"""` docstring，标注所有参数和返回值的生命周期与所有权责任：

| 参数/返回值类型 | docstring 标注 |
|-----------------|---------------|
| `self`（消费，如 `unwrap`, `into_inner`） | `ownership consumed — object invalid after call; Rust frees via Drop` |
| `self`（借用，如 `len`, `get`, `push`） | `borrowed — caller retains ownership; call destroy() to free` |
| AbiClass 值传递参数 | `ownership transferred to Rust — Rust frees via Drop; do not use after call` |
| AbiClass 指针传递参数 | `borrowed — caller retains ownership` |
| 标量参数（`int` 等） | `value parameter — no ownership concerns` |
| 输出指针参数 | `value parameter (output pointer) — no ownership concerns` |
| 返回 AbiClass | `returns new X — caller responsible for calling destroy() to free` |
| 返回标量 | `returns value — no ownership concerns` |
| 返回指针 | `returns borrowed pointer — no ownership, no need to free` |
| void 返回 | `no return value` |

## 异步函数（Futures）

`export_class` 和 `export_lib` 支持异步方法/函数。异步 API 的返回值在 CABI 层面映射为 `Box<dyn Future<Output = R>>`，通过 `HiccRuntime` 执行。

### HiccRuntime trait

用户需实现 `HiccRuntime` trait 来提供异步执行环境：

```rust
pub trait HiccRuntime {
    fn block_on(&self, f: Pin<&mut dyn Future<Output = ()>>);
    fn spawn(&self, f: Pin<&mut dyn Future<Output = ()>>);
}
```

`Box<dyn HiccRuntime>` 通过 `#[export_class(in_hicc)]` 自动导出为 CABI 类型，可直接跨语言传递。

### 获取异步结果：wait 与 async_wait

`Box<dyn Future<Output = R>>` 的方法表提供两种获取结果的方式：

| 方法 | 说明 |
|------|------|
| `wait(runtime)` | 阻塞等待 Future 完成，返回 `R`（self 消费） |
| `async_wait(runtime, notify)` | 异步回调：Future 完成后调用 `notify.on_return(result, ctx)`（self 消费） |

### Notify 回调结构

```rust
#[repr(C)]
pub struct Notify<R> {
    pub on_return: extern "C" fn(<R as AbiType>::OutputType, *const ());
    pub ctx: *const (),
}
```

C 侧定义对应的回调结构体，Python 绑定会生成 `Notify_R` 类。

### export_lib 异步函数

`export_lib` 也支持异步函数：

```rust
async fn make_name(s: &str) -> String {
    format!("{}_suffix", s)
}

#[export_lib(name = "my_lib")]
mod ffi {
    async fn make_name(s: &str) -> String;
}
```

C 侧调用 `make_name(s)` 得到 `AbiClass_Box_dynFuture_Output_String`，再通过 `wait` 或 `async_wait` 获取 `String`。

## 安全约束

- 依赖 `#![feature(specialization)]`
- **调用者应保证在同一个线程调用 Rust 接口及其返回参数的访问**：Rust 类型在跨语言场景下不强制线程绑定，但 `Cell`、`RefCell`、`Mutex`、`RwLock` 等类型依赖于 Rust 的线程安全模型。调用者必须确保对同一个 Rust 对象的所有操作（包括方法调用和返回值的读写）在同一线程中执行，否则可能导致未定义行为。

**Stable Rust 编译方式：**

```bash
RUSTC_BOOTSTRAP=1 cargo build
```

`RUSTC_BOOTSTRAP=1` 允许 stable 编译器使用 unstable feature，效果等同于 nightly。在 CI 脚本和 Makefile 中需确保此环境变量生效：

```bash
export RUSTC_BOOTSTRAP=1
cargo build --features cbindgen
```

需在 Rust 源文件顶部声明：

```rust
#![feature(specialization)]
```

## Feature flags

| Feature | 说明 |
|---------|------|
| `cbindgen` | 启用 `ExportType` trait 和 `TypeRegistry`，用于配合 `hicc-cbindgen` 生成 C/Cython/Python 绑定 |

**为什么 cbindgen 是可选 feature：**

`cbindgen` feature 引入 `ExportType` trait 实现，为每个导出类型生成 cbindgen 元数据。这些元数据仅在运行 `hicc-cbindgen` 生成 C 头文件或 Python 绑定时需要——正常编译和运行 FFI 库不需要。无条件开启会增加二进制体积。因此只在生成绑定文件时启用。

**配置方法：**

下游 crate 的 `Cargo.toml`：

```toml
[features]
cbindgen = ["hicc-rs/cbindgen"]

[dependencies]
hicc-rs = { path = "..." }
```

仅在生成头文件时启用：

```bash
# 生成头文件（启用 cbindgen）
RUSTC_BOOTSTRAP=1 cargo build --features cbindgen

# 正常编译和运行（不启用 cbindgen）
RUSTC_BOOTSTRAP=1 cargo build
```

## 完整示例

| 示例 | 说明 |
|------|------|
| `hicc-rs-examples/basic_lib/` | 函数表导出 + C/Python/Cython FFI |
| `hicc-rs-examples/async-func/` | 异步函数/异步关联方法 + C/Python FFI |
| `hicc-rs-examples/foreign_type/` | foreign 模式封装第三方类型 + C/Python/Cython FFI |
| `hicc-rs-examples/foo_bar_baz/` | 跨 crate 类型别名 + C/Python FFI |
| `hicc-rs-examples/rust-std/` | core/alloc/std 全类型覆盖 + C/Python FFI |
| `hicc-rs-examples/no-std/` | no_std 环境 + C/Python FFI（手写绑定） |

```bash
# Rust 端
RUSTC_BOOTSTRAP=1 cargo build --workspace

# C FFI
cd hicc-rs-examples/foreign_type/c && make run

# Python Cython FFI（需要安装 Cython：pip3 install Cython）
cd hicc-rs-examples/foreign_type/python && make run
```

## 内置类型清单

hicc-rs 为以下 Rust 标准类型预定义了 `ValueType` 和 `#[export_class(in_hicc)]` CABI 封装，可直接跨语言调用。

> **备注：** 用户可通过 `foreign` 属性为任何第三方类型定义自己的操作接口，见 [Foreign 模式](#foreign-模式---封装第三方类型)。

### core — 基础类型

| 类型 | 方法 | 说明 |
|------|------|------|
| `Option<T>` | `is_none(&self) → bool` | 是否为 None |
| | `unwrap(self) → T` | 解包（self 消费），None 时 panic |
| | `take(&mut self) → Option<T>` | 取出内部值，原 Option 变 None |
| | `as_ref(&self) → *const T` | 获取内部值的裸指针，None 返回 null |
| | `as_mut(&mut self) → *mut T` | 获取内部值的可变裸指针，None 返回 null |
| `Result<T, E>` | `is_ok(&self) → bool` | 是否为 Ok |
| | `is_err(&self) → bool` | 是否为 Err |
| | `ok(self) → T` | 提取 Ok 值（self 消费），Err 时 panic |
| | `err(self) → E` | 提取 Err 值（self 消费），Ok 时 panic |
| `[T; N]` (Array) | `len(&self) → usize` | 数组长度（编译期常量 N） |
| | `get(&self, idx: usize) → &T` | 按索引获取元素的引用 |
| | `get_mut(&mut self, idx: usize) → &mut T` | 按索引获取元素的可变引用 |
| | `set(&mut self, idx: usize, val: T)` | 设置指定索引的值 |
| `(T1, T2)` | `field_0(&self) → &T1` / `field_1(&self) → &T2` | 获取字段引用 |
| | `take_0(self) → T1` / `take_1(self) → T2` | 取出字段值（self 消费） |
| `(T1,..,T6)` | `field_N(&self) → &TN` / `take_N(self) → TN` | Tuple2..6 各字段均有 field_N + take_N |
| `&[T]` | `len(&self) → usize` | 切片长度 |
| | `get(&self, idx: usize) → &T` | 按索引获取元素引用 |
| `&str` | `len(&self) → usize` | 字符串长度 |
| | `get(&self, idx: usize) → u8` | 按索引获取字节值 |
| `&mut [T]` | `len(&self) → usize` | 切片长度 |
| | `get(&self, idx: usize) → &T` | 按索引获取元素引用 |
| | `get_mut(&mut self, idx: usize) → &mut T` | 按索引获取可变引用 |
| | `set(&mut self, idx: usize, val: T)` | 设置指定索引的值 |
| `&mut str` | `len(&self) → usize` | 字符串长度 |
| | `get(&self, idx: usize) → u8` | 按索引获取字节值 |
| `&dyn Any` | `type_id(&self) → [u8; 16]` | 获取 `TypeId`（16 字节 POD） |
| `&mut dyn Any` | `type_id(&self) → [u8; 16]` | 获取 `TypeId`（16 字节 POD） |
| `NonNull<T>` | — | ValueType 定义（`Value = IsMut`，C ABI 中等同于 `&mut T`），无 export_class 方法 |
| `Cell<T>` | `set(&self, val: T)` | 设置新值 |
| | `replace(&self, val: T) → T` | 替换并返回旧值 |
| | `into_inner(self) → T` | 消费并返回内部值 |
| | `as_ptr(&self) → *mut T` | 获取内部数据的裸指针 |
| `RefCell<T>` | `replace(&self, val: T) → T` | 替换并返回旧值 |
| | `into_inner(self) → T` | 消费并返回内部值 |
| | `get_mut(&mut self) → &mut T` | 获取可变引用 |
| | `as_ptr(&self) → *mut T` | 获取内部数据的裸指针 |
| `Box<dyn HiccRuntime>` | — | 异步运行时，通过 `#[export_class(in_hicc)]` 导出，仅内置方法表（hicc_destroy 等），详见 [异步函数](#异步函数---futures) |
| `Box<dyn Future<Output=R>>` | `wait(self, runtime) → R` | 阻塞等待 Future 完成（self 消费） |
| | `async_wait(self, runtime, Notify<R>)` | 异步回调（self 消费），详见 [异步函数](#异步函数---futures) |

### alloc — 堆分配类型

| 类型 | 方法 | 说明 |
|------|------|------|
| `String` | `len(&self) → usize` | 字符串长度 |
| | `push_str(&mut self, s: &str)` | 追加 `&str`（参数按值传递 = 所有权转移） |
| | `push_cstr(&mut self, s: *const i8)` | 追加 C 字符串（null-terminated） |
| | `insert_str(&mut self, idx: usize, s: &str)` | 在指定位置插入 `&str` |
| | `insert_cstr(&mut self, idx: usize, s: *const i8)` | 在指定位置插入 C 字符串 |
| | `as_str(&self) → &str` | 获取 `&str` 引用 |
| | `as_bytes(&self) → &[u8]` | 获取字节切片引用 |
| `Vec<T>` | `len(&self) → usize` | 向量长度 |
| | `push(&mut self, val: T)` | 尾部追加元素（IsClass 类型按值传递 = 所有权转移） |
| | `pop(&mut self) → Option<T>` | 尾部弹出元素 |
| | `get(&self, idx: usize) → &T` | 按索引获取元素引用 |
| | `get_mut(&mut self, idx: usize) → &mut T` | 按索引获取可变引用 |
| | `as_slice(&self) → &[T]` | 获取切片引用 |
| `Box<T>` | `get(&self) → &T` | 获取内部值引用（仅 T 为值类型时有效） |
| | `get_mut(&mut self) → &mut T` | 获取内部值可变引用（仅 T 为值类型时有效） |
| | — | T 为 class 类型时（如 `Box<String>`），Abi 类型等同于 `AbiClass<T>`，直接用 T 的方法表（`len`/`as_str`/`push_cstr` 等） |
| `Rc<T>` | `get(&self) → &T` | 获取内部值引用 |
| `Arc<T>` | `get(&self) → &T` | 获取内部值引用 |
| `BTreeMap<K, V>` | `len(&self) → usize` | 元素数量 |
| | `is_empty(&self) → bool` | 是否为空 |
| | `contains_key(&self, key: &K) → bool` | 是否包含指定键 |
| | `get(&self, key: &K) → Option<&V>` | 查找值（返回 Option<&V>） |
| | `insert(&mut self, key: K, val: V) → Option<V>` | 插入键值对 |
| | `remove(&mut self, key: &K) → Option<V>` | 删除指定键 |
| | `iter(&self) → Iter<'_, K, V>` | 获取迭代器 |
| | `iter_mut(&mut self) → IterMut<'_, K, V>` | 获取可变迭代器 |
| | `into_iter(self) → IntoIter<K, V>` | 获取消费迭代器（self 消费） |
| `BTreeMap::Iter<'_, K, V>` | `next(&mut self) → Option<(&K, &V)>` | 迭代器前进 |
| `BTreeMap::IterMut<'_, K, V>` | `next(&mut self) → Option<(&K, &mut V)>` | 可变迭代器前进 |
| `BTreeMap::IntoIter<K, V>` | `next(&mut self) → Option<(K, V)>` | 消费迭代器前进 |
| `BTreeSet<T>` | `len(&self) → usize` | 元素数量 |
| | `is_empty(&self) → bool` | 是否为空 |
| | `contains(&self, val: &T) → bool` | 是否包含指定值 |
| | `insert(&mut self, val: T) → bool` | 插入值 |
| | `remove(&mut self, val: &T) → bool` | 删除指定值 |
| | `iter(&self) → Iter<'_, T>` | 获取迭代器 |
| | `into_iter(self) → IntoIter<T>` | 获取消费迭代器（self 消费） |
| `BTreeSet::Iter<'_, T>` | `next(&mut self) → Option<&T>` | 迭代器前进 |
| `BTreeSet::IntoIter<T>` | `next(&mut self) → Option<T>` | 消费迭代器前进 |

### std — 标准库类型（需 `std` feature）

| 类型 | 方法 | 说明 |
|------|------|------|
| `HashMap<K, V>` | 同 `BTreeMap<K, V>` | 需 `K: Hash + Eq` |
| `HashMap::Iter<'_, K, V>` | `next(&mut self) → Option<(&K, &V)>` | |
| `HashMap::IterMut<'_, K, V>` | `next(&mut self) → Option<(&K, &mut V)>` | |
| `HashMap::IntoIter<K, V>` | `next(&mut self) → Option<(K, V)>` | |
| `HashSet<T>` | 同 `BTreeSet<T>` | 需 `T: Hash + Eq` |
| `HashSet::Iter<'_, T>` | `next(&mut self) → Option<&T>` | |
| `HashSet::IntoIter<T>` | `next(&mut self) → Option<T>` |
| `OnceLock<T>` | `get(&self) → Option<&T>` | 获取已初始化的值 |
| | `set(&self, val: T) → Result<(), T>` | 初始化（仅一次） |
| | `into_inner(self) → Option<T>` | 消费并返回内部值 |
| `Mutex<T>` | `lock(&self) → MutexGuard<T>` | 加锁，返回 guard |
| | `try_lock(&self) → Option<MutexGuard<T>>` | 尝试加锁 |
| | `into_inner(self) → T` | 消费并返回内部值 |
| | `get_mut(&mut self) → &mut T` | 获取可变引用（已持有锁） |
| | `is_poisoned(&self) → bool` | 是否中毒 |
| `MutexGuard<'_, T>` | `get(&self) → &T` | 获取引用 |
| | `get_mut(&mut self) → &mut T` | 获取可变引用 |
| `RwLock<T>` | `read(&self) → RwLockReadGuard<T>` | 获取读锁 |
| | `write(&self) → RwLockWriteGuard<T>` | 获取写锁 |
| | `into_inner(self) → T` | 消费并返回内部值 |
| | `get_mut(&mut self) → &mut T` | 获取可变引用（已持有锁） |
| | `is_poisoned(&self) → bool` | 是否中毒 |
| `RwLockReadGuard<'_, T>` | `get(&self) → &T` | 获取引用 |
| `RwLockWriteGuard<'_, T>` | `get(&self) → &T` | 获取引用 |
| | `get_mut(&mut self) → &mut T` | 获取可变引用 |

> **注意：** **调用者应保证在同一个线程调用 Rust 接口及其返回参数的访问**。Rust 的 `Cell`、`RefCell`、`Mutex`、`RwLock` 等类型依赖于线程安全检查（`Send`/`Sync`），跨语言调用场景下，调用者必须手动确保对同一个 Rust 对象的所有操作（包括方法调用和返回值的读写）在同一线程中执行，否则可能导致未定义行为。
