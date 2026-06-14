# hicc-cbindgen

C 头文件与 Python 绑定生成工具。输入一个使用 `#[export_lib]` / `#[export_class]` 的 Rust 库 crate，输出对应的 C 头文件或 Python ctypes 绑定。

## API 文档

运行 `cargo doc --manifest-path hicc-cbindgen/Cargo.toml` 后，可在 `../target/doc/hicc_cbindgen/` 浏览完整 API 文档。关键入口：

| 函数 | 说明 |
|------|------|
| [`generate_python`](../target/doc/hicc_cbindgen/fn.generate_python.html) | 从 Rust 源码生成 Python ctypes 绑定 |
| [`parse_c_header_structs`](../target/doc/hicc_cbindgen/fn.parse_c_header_structs.html) | 解析 C 头文件中的跨 crate POD 结构体 |

## 工作原理

```
输入: Rust 库 crate
  │
  ▼
[1] cargo expand --features cbindgen
  │   展开宏，发现 *_cbindgen 函数
  │
  ▼
[2] 创建临时 helper crate
  │   调用 *_cbindgen 收集类型定义 → 输出 Rust 源码
  │
  ▼
[3] cbindgen 解析 Rust 源码 → 输出 C 头文件
  │   或 syn 解析 Rust 源码 → 输出 Python 绑定
```

1. 以 `cbindgen` feature 对目标 crate 执行 `cargo expand`，展开过程宏后找到所有 `*_cbindgen` 函数
2. 创建一个临时 crate 依赖目标 crate，调用这些函数收集所有 `AbiClass`、`AbiMethods` 类型的 Rust `#[repr(C)]` 定义
3. C 头文件：使用 [`cbindgen`](https://crates.io/crates/cbindgen) 将收集到的 Rust 代码解析为 C 头文件
4. Python 绑定：使用 [`syn`](https://crates.io/crates/syn) 将 Rust 源码解析为 Python ctypes 绑定（包含包装器类和工厂函数）

## 安装

```bash
# 方式一：从源码编译安装（推荐）
cargo install --path hicc-cbindgen

# 方式二：从 git 仓库安装
cargo install --git <repo-url> hicc-cbindgen

# 方式三：仅构建，产物在 target/debug/hicc-cbindgen
cargo build --manifest-path hicc-cbindgen/Cargo.toml
```

## 使用

```bash
hicc-cbindgen -c <crate-path> [-o <output>] [-l <language>] [--lib <lib-name>]
```

| 参数 | 说明 | 默认值 |
|------|------|--------|
| `-c <path>` | 目标 crate 目录（必须包含 `Cargo.toml`） | `.` |
| `-o <path>` | 输出文件路径 | stdout |
| `-l <lang>` | 输出语言：`c` / `cxx` / `cython` / `python` | `c` |
| `--lib <name>` | 动态库文件名（`-l python` 时可选，缺省从 Cargo.toml 推导） | `lib<crate_name>.so` |

**`--lib` 缺省推导规则**：当使用 `-l python` 但未指定 `--lib` 时，从目标 crate 的 `Cargo.toml` 读取包名（`package.name`），将 `-` 替换为 `_`，拼成 `lib<name>.so`。例如 crate 名为 `example-basic_lib`，则缺省库名为 `libexample_basic_lib.so`。若 Cargo.toml 无法提供库名，则报错要求手动指定 `--lib`。

### C 头文件示例

```bash
# 打印到 stdout
hicc-cbindgen -c examples/basic_lib

# 写入文件
hicc-cbindgen -c examples/basic_lib -o hicc_demo.h
```

### Python 绑定示例

```bash
# 生成 Python 绑定（--lib 可省略，自动从 Cargo.toml 推导）
hicc-cbindgen -c examples/basic_lib -l python -o hicc_demo.py

# 生成 Python 绑定（手动指定库名）
hicc-cbindgen -c examples/basic_lib -l python --lib=libexample_basic_lib.so -o hicc_demo.py
```

Python 绑定生成三个层次的代码：

1. **低级 ctypes 结构体** — `AbiClass_*` / `AbiMethods_*` 的 `#[repr(C)]` 映射
2. **包装器类** — 提供 Python 风格的方法接口；**必须显式调用 `destroy()` 释放 Rust 堆内存**（见下文）
3. **模块级工厂函数** — 对应 `#[export_lib]` 中声明的函数

## ⚠ destroy() 必须显式调用

Python 的 `__del__` 调用时机由 GC 决定，**不可靠且不安全**：

- GC 可能延迟回收，导致 Rust 堆内存长期不释放
- GC 在进程退出时可能跳过 `__del__`，导致内存泄漏
- 在引用循环、异常等场景下 `__del__` 可能根本不被调用
- `__del__` 中调用 FFI 的 `hicc_destroy` 可能触发段错误（Python 解释器正在关闭）

**因此，生成的包装器类不依赖 `__del__` 进行析构。用户必须在使用完毕后显式调用 `destroy()`：**

```python
# ✗ 错误 — 依赖 GC 自动回收，可能泄漏 Rust 内存
s = new_str()
result = s.len()

# ✓ 正确 — 显式调用 destroy()
s = new_str()
result = s.len()
s.destroy()

# ✓ 正确 — consumes_self 的方法会自动 nullify _inner，但仍需注意
opt = new_option(42)
val = opt.unwrap()  # self 被 consume，_inner 自动置 None
# opt.destroy() 不需要 — unwrap 已消费 self，Rust 侧 Drop 释放
```

**规则**：
- 值传递 `self` 的方法（如 `unwrap`, `into_inner`, `take_0`）会消费 `self`，`_inner` 自动置 `None`，不需要再调用 `destroy()`
- 借用 `self` 的方法（如 `len`, `get`, `push`）不消费 `self`，**使用完毕后必须调用 `destroy()`**
- 返回新 AbiClass 的方法/函数，**返回的对象也必须调用 `destroy()`**

## Python 绑定中的所有权注释

每个方法和工厂函数**始终生成** `"""Ownership"""` docstring，标注所有参数和返回值的生命周期与所有权责任：

### 方法参数

| 参数类型 | docstring 标注 |
|----------|---------------|
| `self`（值传递 / consumes_self，如 `unwrap`, `take_0`） | `self: ownership consumed — object invalid after call; Rust frees via Drop` |
| `self`（指针传递 / 借用，如 `len`, `get`, `push_str`） | `self: borrowed — caller retains ownership; call destroy() to free` |
| `self`（无 FFI 参数，如 `size_of`） | `self: borrowed — caller retains ownership; call destroy() to free` |
| AbiClass 值传递（如 `push(val: T)`） | `arg: ownership transferred to Rust — Rust frees via Drop; do not use arg after call` |
| AbiClass 指针传递（如 `push_str(&mut self, s: &str)`） | `arg: borrowed — caller retains ownership` |
| 标量参数（如 `int`） | `arg: value parameter — no ownership concerns` |
| 输出指针参数（`ctypes.POINTER(ctypes.c_*)`） | `arg: value parameter (output pointer) — no ownership concerns` |

### 返回值

| 返回类型 | docstring 标注 |
|----------|---------------|
| AbiClass（新对象，如 `make_unique`, `new_string`） | `returns new X — caller responsible for calling destroy() to free` |
| 标量（如 `len → usize`） | `returns value — no ownership concerns` |
| 指针（如 `as_ref → *const T`） | `returns borrowed pointer — no ownership, no need to free` |
| void / None | `no return value` |

### 示例输出

```python
class ForeignType_String:
    """Wrapper for AbiClass_String — MUST call destroy() to free Rust heap memory.

    Python's __del__ timing is unpredictable and NOT safe for releasing
    Rust objects. You must explicitly call destroy() when done with this
    object, or risk leaking Rust heap memory.
    """

def push(self, arg0: ForeignType_String):
    """Ownership:
    - `self`: borrowed — caller retains ownership; call destroy() to free
    - `arg0`: ownership transferred to Rust — Rust frees via Drop; do not use `arg0` after call
    - no return value
    """

def make_unique(self):
    """Ownership:
    - `self`: ownership consumed — object invalid after call; Rust frees via Drop
    - returns new `ForeignType_Vec_String` — caller responsible for calling destroy() to free
    """

def add(arg0: int, arg1: int):
    """Ownership:
    - `arg0`: value parameter — no ownership concerns
    - `arg1`: value parameter — no ownership concerns
    - returns value — no ownership concerns
    """
```

## 依赖

- 目标 crate 必须依赖 `hicc-rs` 并启用 `cbindgen` feature（或通过 `[features]` 透传）
- 需要安装 `cargo-expand`（`cargo install cargo-expand`）
- 需要 Rust nightly（或设置 `RUSTC_BOOTSTRAP=1`）

## 集成到构建

### C 头文件

参考 `hicc-rs-examples/basic_lib/c/Makefile`：

```makefile
HEADER := hicc_demo.h
CBINDGEN := path/to/hicc-cbindgen

$(HEADER):
    RUSTC_BOOTSTRAP=1 $(CBINDGEN) -c examples/basic_lib > $(HEADER)
```

### Python 绑定

参考 `hicc-rs-examples/basic_lib/python/Makefile`：

```makefile
PYBIND := hicc_demo.py
CBINDGEN := path/to/hicc-cbindgen

$(PYBIND):
    RUSTC_BOOTSTRAP=1 $(CBINDGEN) -c examples/basic_lib \
        -l python -o $(PYBIND)
```

**注意**：`--lib` 现在可选。若 Makefile 中显式指定了库名（如 `--lib=libexample_basic_lib.so`），可保留不变；若库名与 Cargo.toml 包名一致，可省略 `--lib`。

### Cython 绑定

参考 `hicc-rs-examples/basic_lib/cython/Makefile`：

```makefile
PXD := hicc_demo.pxd
HEADER := hicc_demo.h
CBINDGEN := path/to/hicc-cbindgen

$(PXD) $(HEADER):
    RUSTC_BOOTSTRAP=1 $(CBINDGEN) -c examples/basic_lib -l cython -o $(PXD)
```