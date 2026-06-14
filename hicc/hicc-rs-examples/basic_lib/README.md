# basic_lib — hicc-rs C FFI 示例

该示例演示了 **hicc-rs** 框架的核心功能：通过 `#[export_lib]` 过程宏将一组 Rust 函数暴露为 C ABI，配合 `hicc-cbindgen` 自动生成 C 头文件，实现 Rust 库的零手工 C 绑定。

## 支持的参数／返回类型

| 类型 | Rust | C 侧 | 所有权 |
|------|------|------|--------|
| 原生整数 | `i32`, `i64` 等 | `int32_t`, `int64_t` | 值传递，无生命周期管理 |
| 自定义容器 | `Container<T>` | `AbiClass_Container_i32` | 堆分配，需调用 `destroy` |
| Option | `Option<T>` | `AbiClass_Option_i32` | 堆分配，需调用 `destroy` |
| 字符串 | `&'static str` | `AbiClass_str` | 堆分配，需调用 `destroy` |
| 切片 | `&'static [T]` | 对应 `AbiClass` | 堆分配，需调用 `destroy` |
| 定长数组 | `[T; N]` | 对应 `AbiClass` | 堆分配，需调用 `destroy` |
| `#[repr(C)]` 结构体 | `Point` | `Point` | POD，值传递 |

## 目录结构

```
examples/basic_lib/
├── Cargo.toml              # Rust 包配置（cdylib + rlib）
├── README.md               # 本文件
├── src/
│   ├── lib.rs              # #[export_lib] 定义 Rust API
│   └── main.rs             # Rust 二进制入口（调用同一套 API）
├── c/
│   ├── main.c              # C 示例程序
│   ├── Makefile            # 一键构建 cdylib → 头文件 → C → 运行
│   └── hicc_demo.h         # (自动生成) C 头文件
└── python/
    ├── test_demo.pyx       # Python Cython 示例程序（依赖 hicc_demo.pxd）
    ├── setup.py            # Cython 编译脚本
    ├── Makefile            # 一键构建 cdylib → .pxd → Cython → 运行
    └── hicc_demo.pxd       # (自动生成) Cython 定义文件

hicc-cbindgen/              # 位于 ../../hicc-cbindgen/
```

## 前置条件

- **Rust nightly**（或设置 `RUSTC_BOOTSTRAP=1`，因依赖 `#![feature(specialization)]`）
- **gcc / clang**
- **Python 3 + Cython**（`pip3 install Cython`）
- **hicc-cbindgen**（自动编译，位于仓库根目录的 `hicc-cbindgen/`）

## 运行

### Rust 二进制

```bash
RUSTC_BOOTSTRAP=1 cargo run --features cbindgen -p example-basic_lib
```

### C FFI 程序（完整流程）

```bash
cd c/
make          # 构建 cdylib → hicc-cbindgen 生成头文件 → gcc 编译
make run      # 运行 C 程序
make clean    # 清理产物
```

`make` 的执行流程：
1. `cargo build -p example-basic_lib` → 生成 `libexample_basic_lib.so`
2. `hicc-cbindgen -c ../` → 扫描 `#[export_lib]` 宏，生成 `hicc_demo.h`
3. `gcc main.c -lexample_basic_lib` → 编译 C 程序链接到动态库

### Python Cython 测试

```bash
cd python/
make          # 构建 cdylib → hicc-cbindgen 生成 .pxd → Cython 编译
make run      # 运行 Python 测试
make clean    # 清理编译产物
make distclean  # 清理全部（包括自动生成的 .pxd）
```

`make` 的执行流程：
1. `cargo build -p example-basic_lib` → 生成 `libexample_basic_lib.so`
2. `hicc-cbindgen -c ../ -l cython` → 扫描 `#[export_lib]` 宏，生成 `hicc_demo.pxd`
3. `python3 setup.py build_ext --inplace` → 编译 `test_demo.pyx` → `test_demo.cpython-*.so`

Python 测试使用 **Cython** 编译，通过 `cimport hicc_demo` 直接使用 `.pxd` 中的 C 级别类型定义。所有 struct 布局、方法表签名和函数声明均由 `hicc-cbindgen` 自动维护，与 C 头文件保持同步。

**一步步手动构建：**

```bash
# 1. 构建 Rust cdylib
RUSTC_BOOTSTRAP=1 cargo build -p example-basic_lib

# 2. 生成 Cython 定义文件
hicc-cbindgen -c examples/basic_lib -l cython -o examples/basic_lib/python/hicc_demo.pxd

# 3. 编译 Cython 扩展
cd examples/basic_lib/python
python3 setup.py build_ext --inplace

# 4. 运行
LD_LIBRARY_PATH=../../target/debug python3 -c "import test_demo; test_demo.main()"
```

## 工作原理

### 1. Rust 侧：`#[export_lib]`

```rust
#[export_lib(name = "demo")]
mod lib {
    fn add(x: i32, y: i32) -> i32 { x + y }

    fn new_container(x: i32) -> super::Container<i32> {
        super::Container(x)
    }

    fn container_value(x: super::Container<i32>) -> i32 {
        *x.get()
    }
    // ...
}
```

宏展开生成：
- 一个 `extern "C"` 函数表结构体 `Hicc_demo`，包含所有方法的函数指针
- 一个 `extern "C" fn demo() -> *const Hicc_demo` 入口函数
- 请求 `#[export_class]` 标记的类型生成对应的 `AbiClass_*` 和方法表

### 2. C 侧：AbiClass 对象模型

每个堆分配的 Rust 类型在 C 侧表示为 `AbiClass_<T>` 结构体：

```c
typedef struct AbiClass_Container_i32 {
    const struct HiccContainerMethods_i32 *methods;  // 虚方法表
    const void *this_;                                // 内部数据指针
    uintptr_t level;                                  // 指针重数（嵌套层数）
} AbiClass_Container_i32;
```

### 3. 所有权与 destroy

所有通过工厂函数返回的 `AbiClass` 都是堆分配的，必须调用 `destroy` 释放：

#### ✅ 正确的销毁模式

通过 **methods 虚表** 读取数据（不转移所有权），然后显式销毁：

```c
struct AbiClass_Container_i32 c = fn->new_container(42);
int32_t val = *c.methods->get(&c);   // 借用，C 保留所有权
c.methods->destroy(c);               // 显式释放
```

#### ❌ 错误的销毁模式

**切勿**在按值传给 Rust 函数后再调用 destroy：

```c
struct AbiClass_Container_i32 c = fn->new_container(42);
fn->container_value(c);              // 按值传递 → 所有权已转移给 Rust
c.methods->destroy(c);               // ❌ double-free！
```

按值传递后，Rust 侧的 `Drop` 已经释放了堆内存，C 侧的指针已悬空。

## 生成的 C API

`demo()` 返回一个包含所有函数指针的结构体：

```c
const struct Hicc_demo *demo(void);

struct Hicc_demo {
    // -- 原生类型 --
    int32_t (*add)(int32_t, int32_t);
    int32_t (*negate)(int32_t);

    // -- 消费者（按值转移所有权） --
    int32_t (*container_value)(struct AbiClass_Container_i32);
    int64_t (*double_option)(struct AbiClass_Option_i32);
    uintptr_t (*check_str)(struct AbiClass_str);
    uintptr_t (*count_some)(struct AbiClass_Option_i32);
    uintptr_t (*total_len)(struct AbiClass_str_3);
    struct Point (*add_point)(struct Point, struct Point);

    // -- 工厂函数（返回新对象，C 获得所有权） --
    struct AbiClass_Container_i32 (*new_container)(int32_t);
    struct AbiClass_Option_i32 (*new_option)(int32_t);
    struct AbiClass_str (*new_str)(void);
    struct AbiClass_Option_i32 (*new_slice)(void);
    struct AbiClass_str_3 (*new_array)(void);
};
```

### 各类型的 methods 虚表方法

**Container\<i32\>** (`HiccContainerMethods_i32`)：
- `destroy`, `make_unique`, `make_ref_mut`, `size_of`, `write`, `make_ref`
- `get(&self) -> *const i32`

**Option\<i32\>** (`HiccOptionMethods_i32`)：
- `destroy`, `make_unique`, `make_ref_mut`, `size_of`, `write`, `make_ref`
- `is_none(&self) -> bool`
- `unwrap(self) -> i32`
- `take(&mut self) -> Option<i32>`
- `as_ref(&self) -> *const i32`
- `as_mut(&mut self) -> *mut i32`

**Str** (`&'static str`, `HiccStrMethods`)：
- `destroy`, `make_unique`, `make_ref_mut`, `size_of`, `write`, `make_ref`
- `len(&self) -> usize`
- `get(&self, index) -> u8`

**Array\<&str, 3\>** (`HiccArrayMethods_str_3`)：
- `destroy`, `make_unique`, `make_ref_mut`, `size_of`, `write`, `make_ref`
- `len(&self) -> usize`
- `set(&mut self, index, value)`
- `get(&self, index) -> AbiClass_str`
- `get_mut(&self, index) -> AbiClass_str`

## 完整示例速览

```c
#include "hicc_demo.h"

int main(void) {
    const struct Hicc_demo *fn = demo();

    // 原生类型 — 值传递
    printf("%d\n", fn->add(3, 4));

    // 堆分配 — 通过 vtable 读值 + 显式 destroy
    {
        struct AbiClass_Container_i32 c = fn->new_container(42);
        printf("%d\n", *c.methods->get(&c));
        c.methods->destroy(c);
    }

    // 同样调用 destroy
    {
        struct AbiClass_str s = fn->new_str();
        printf("%zu\n", s.methods->len(&s));
        s.methods->destroy(s);
    }

    // 按值传递给 Rust 函数 — 所有权自动转移
    {
        struct AbiClass_Option_i32 opt = fn->new_option(99);
        printf("%" PRId64 "\n", fn->double_option(opt));  // Rust 负责释放
    }

    return 0;
}
```
