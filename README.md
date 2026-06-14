# hicc-usages

> 基于 [hicc](hicc/) 的 **C++ → Rust FFI 自动生成**端到端示例集与工具链。
> 给定任意 C++ 项目，自动产出可编译运行的 Rust crate。

## 当前状态

- **🎉 48/48 特性端到端通过**：C++ 编译 ✅ → AST 提取 ✅ → Rust 生成 ✅ → Rust 冒烟测试 ✅
- **rust_gen 100% 自动化**：48 个 crate 的 `lib.rs` / `smoke.rs` / `build.rs` / `Cargo.toml` 全部由工具自动生成
- 三平台代码已就位（Linux 实测通过；macOS/Windows 仅链接库差异）
- CI 已配置：`.github/workflows/ci.yml`

## 一键运行

```bash
make                    # 全流程：cpp-build → rust-gen → rust-test（48 个特性端到端）
make verify N=001       # 单特性端到端
make gen-build          # 从 tools/scaffold/ 模板重新生成所有示例的 Makefile/CMakeLists.txt
make clean              # 清理所有缓存
```

**其他目标：** `cpp` / `rust` / `cpp-build` / `rust-gen` / `rust-test` / `new N=NNN NAME=...` / `help`

## 流水线

```
examples/NNN_<name>/cpp/        ← 手写的纯 C++ 项目（Makefile + CMakeLists.txt）
                │
                ▼
tools/ast-extract/extract.sh    ← clang -E 预处理 + clang -ast-dump=json
                │
                ▼
examples/NNN_<name>/ast/        ← preprocessed.cpp + ast.json + symbols.json
                │
                ▼
tools/rust-gen/rust_gen.py      ← 解析 symbols.json + Jinja2 模板渲染
                │
                ▼
examples/NNN_<name>/rust/       ← Cargo.toml + build.rs + src/lib.rs + tests/smoke.rs
                ▼
cargo test                      ← build.rs 自动调用 cmake/make 构建 C++ 库 + 链接
```

## 目录结构

```
hicc-usages/
├── hicc/                                # 只读子模块（hicc FFI 框架，禁止修改）
├── examples/NNN_<name>/                 # 一个 C++ 特性 = 一个目录
│   ├── cpp/                             # 手写的独立 C++ 项目
│   │   ├── include/hicc_usages/<name>.h # 纯 C++ API（不写 extern "C"）
│   │   ├── src/<name>.cpp               # 实现
│   │   ├── test/main.cpp                # C++ 端验证程序
│   │   ├── Makefile + CMakeLists.txt    # 由 tools/scaffold/ 模板自动生成（make gen-build）
│   ├── ast/                             # 工具产出（gitignore）
│   └── rust/                            # 工具产出
│       ├── Cargo.toml / build.rs / src/lib.rs / tests/smoke.rs
├── tools/
│   ├── ast-extract/                     # Bash + Python（extract.sh + symfilter.py）
│   ├── rust-gen/                        # rust_gen.py + special_handlers.py + filter.py + mapping.py + special.yaml + render/
│   └── scaffold/                        # Makefile.tmpl + CMakeLists.txt.tmpl
├── scripts/                             # cpp-build-all.sh / rust-gen-all.sh / rust-test-all.sh / verify-one.sh / scaffold.sh / gen-build-files.sh / run-pipeline-chunk.sh / _common.sh
└── README.md
```

## 特性支持总表

**2 种处理模式：**

| 标记 | 模式 | 含义 |
|------|------|------|
| ✅ 直接 | 直接路径 | rust_gen 自动产出 active 代码，crate 默认即可编译运行（含 `#[cpp(...)]` 属性直接生成、`hicc::cpp!` 块内活跃注入 typedef/factory、filter 内置跳过等所有自动行为） |
| 💬 注释式注入 | lib.rs 内注释 | rust_gen 在 `lib.rs` 中以注释形式产出建议包装代码；用户取消注释即启用，不取消对 crate 无影响 |

**48 个特性一览：**

| # | 特性 | 模式 | 关键说明 |
|---|------|------|---------|
| 001 | hello_world | ✅ 直接 | 自由函数 + 简单类 Greeter |
| 002 | function_overload | ✅ 直接 | 同名重载自动加参数类型后缀（`add_int_int`） |
| 003 | default_args | ✅ 直接 | C++ 端展开为多个函数，按重载处理 |
| 004 | inline_functions | ✅ 直接 | inline 对 FFI 透明 |
| 005 | variadic_functions | ✅ 直接 | C++ 端预固定参数数量（`sum_2/sum_3/sum_4`） |
| 006 | class_basic | ✅ 直接 | `class Counter { get/inc/dec/reset }` |
| 007 | class_constructor | ✅ 直接 | 私有 ctor + `static T* create(args)` |
| 008 | class_copy | ✅ 直接 | `clone()` 命名替代拷贝 ctor |
| 009 | class_move | ✅ 直接 | `take_from()` 命名替代移动 ctor |
| 010 | class_static | ✅ 直接 | static 方法转 import_lib 自由函数 |
| 011 | class_const | ✅ 直接 | const → `&self`，非 const → `&mut self` |
| 012 | class_volatile | ✅ 直接 | 按 `&self` 处理（Rust 无 volatile） |
| 013 | inheritance_single | ✅ 直接 | 单继承：每个类独立 import_class |
| 014 | inheritance_multiple | ✅ 直接 | 多继承：同上，hicc 把继承展平 |
| 015 | virtual_basic | ✅ 直接 | 虚函数按普通方法处理（vtable 由 C++ 管理） |
| 016 | virtual_pure | ✅ 直接 | 抽象类无 factory；派生类各自 import |
| 017 | virtual_override | ✅ 直接 | override 按普通方法处理 |
| 018 | virtual_diamond | ✅ 直接 | 虚继承菱形：派生类各自 import_class |
| 019 | operator_overload | 💬 注释式注入 | C++ 保留原始 `operator+/-/* ==`；rust_gen 在 lib.rs 注释中自动产出 `vec2_add/sub/eq` 命名包装供用户取消注释启用 |
| 020 | friend_function | ✅ 直接 | filter 默认跳过含 class 指针参数的 friend 函数；仅类方法导出 |
| 021 | explicit_ctor | ✅ 直接 | `explicit` + 静态 create 替代 |
| 022 | mutable_member | ✅ 直接 | mutable 字段对 filter 透明 |
| 023 | typeid_rtti | ✅ 直接 | filter 默认跳过含基类指针参数的方法；C++ 端用 `type_name()` 命名 |
| 024 | template_function | ✅ 直接 | `#[cpp(func = "int max_of<int>(int, int)")]`，hicc-build 透传模板语法 |
| 025 | template_class | ✅ 直接 | C++ 纯 `template<typename T> class Stack`；rust_gen 在 `hicc::cpp!` 块自动注入 `using IntStack = Stack<int>` + namespace 级 `create_int_stack/free_int_stack` |
| 026 | template_specialization | ✅ 直接 | C++ 纯 `template + 特化`；rust_gen 自动注入 `type_name_int()` 等命名空间级包装函数调用 `TypeInfo<int>::name()` |
| 027 | template_instantiation | ✅ 直接 | 同 025，多类型实例化（int + double） |
| 028 | variadic_template | ✅ 直接 | C++ 纯变参模板；rust_gen 自动注入固定参数包装函数（`sum_two/sum_three` 等）调用原变参模板 |
| 029 | unique_ptr | ✅ 直接 | PImpl 模式：Impl 嵌套类通过 `parentDeclContextId` 跳过 |
| 030 | shared_ptr | ✅ 直接 | 同 029 |
| 031 | custom_deleter | ✅ 直接 | 同 029 + 自定义 deleter |
| 032 | placement_new | ✅ 直接 | `operator new(size_t, void*)` + Buffer 包装类 |
| 033 | raii_pattern | ✅ 直接 | 析构由 `free()` 桥接 |
| 034 | vector_basic | ✅ 直接 | `std::vector<int>` 字段私有，暴露 POD 包装类 IntVector |
| 035 | map_basic | ✅ 直接 | 同 034，包装 IntMap |
| 036 | string_basic | ✅ 直接 | `std::string` 字段私有，用 `const char*` C 接口（不用 hicc-std） |
| 037 | array_basic | ✅ 直接 | 同 034，包装 FixedArray |
| 038 | tuple_basic | ✅ 直接 | 同 034，包装 Triple 提供 getter/setter |
| 039 | lambda_basic | ✅ 直接 | lambda 不出现在 AST 顶层；C++ 端用 lambda 实现 + 命名包装 |
| 040 | std_function | ✅ 直接 | `std::function` 字段私有；`set_mode` / `call` 等方法 import_class |
| 041 | functional_bind | ✅ 直接 | `std::bind` 不在 AST 顶层；暴露 `configure` / `next` |
| 042 | exception_basic | ✅ 直接 | try/catch + `thread_local` 错误码；`last_error()` 自由函数 |
| 043 | namespace_nested | ✅ 直接 | rust_gen 折叠命名空间到 Rust 短名 |
| 044 | enum_class | ✅ 直接 | C++ 端提供 `color_to_int` / `int_to_color` 转换函数 |
| 045 | union_basic | 💬 注释式注入 | C++ 保留原始 `union Value`；rust_gen 在 lib.rs 注释中自动产出 `ValueBox` 包装类供用户取消注释启用 |
| 046 | constexpr_basic | ✅ 直接 | constexpr 对 FFI 透明，按普通函数处理 |
| 047 | noexcept_basic | ✅ 直接 | 自由函数保留 noexcept（hicc 支持）；成员方法移除 noexcept（hicc-build 类型匹配限制），C++ 端 1 处调整 |
| 048 | summary | ✅ 直接 | 综合：enum + unique_ptr + exception + namespace + class |

**统计：**

| 模式 | 数量 | 占比 |
|------|------|------|
| ✅ 直接 | 46 | 95.8% |
| 💬 注释式注入 | 2 | 4.2%（019、045） |

## 直接 vs 注入的判定（POC 实测）

hicc-build 解析 `#[cpp(...)]` 字符串时，tokenizer 对某些 C++ 语法无能为力。下表是 POC 验证后的硬性边界及对策：

| C++ 构造 | 处理方式 | 原因 |
|---------|---------|------|
| 函数模板调用 | ✅ 直接 | hicc-build 接受 `max_of<int>(int, int)` 语法 |
| 类模板成员方法 | ✅ 直接 | rust_gen 在 `hicc::cpp!` 块注入 `using Alias = Stack<int>` + namespace 级 factory |
| 类模板静态方法（特化） | ✅ 直接 | rust_gen 在 `hicc::cpp!` 块注入 namespace 级包装函数调用 `TypeName<int>::method()` |
| 变参函数模板 | ✅ 直接 | rust_gen 在 `hicc::cpp!` 块注入 namespace 级固定参数数量包装函数 |
| `operator+` 等运算符 | 💬 注释式注入 | `+` 不是 path 字符，rust_gen 生成命名包装建议供用户取消注释 |
| 联合体（union） | 💬 注释式注入 | hicc 无 raw memory API，rust_gen 生成 ValueBox 包装类建议 |
| noexcept 成员方法 | ✅ 直接（C++ 端 1 处调整） | hicc-build 类型匹配限制；C++ 端从成员方法移除 noexcept |

## 两种自动化机制

1. **✅ 直接**：rust_gen 自动产出 active 代码，crate 默认即可编译运行。涵盖：
   - `#[cpp(...)]` 属性直接生成（大多数特性）
   - `hicc::cpp!` 块内活跃注入 typedef/factory/包装函数（025-028 模板）
   - filter.py 内置跳过规则（020 friend、023 typeid 基类指针参数）
2. **💬 注释式注入**：rust_gen 在 `lib.rs` 中以注释形式产出建议包装代码；用户取消注释即启用，不取消对 crate 无影响（019 operator、045 union）

## 关键设计决策

1. **纯 C++ API**：C++ 头文件不写 `extern "C"`，使用原生 C++（class、template、namespace）。hicc 自动处理 ABI。
2. **类自带 factory**：每个类必须提供 `static T* create(args...)` 和 `static void free(T*)`，规避 hicc 对有参构造的限制。
3. **build.rs 自动构建**：cargo build 时自动调用 cmake（回退 make）编译 C++ 项目，再链接到 `lib<name>.a`。无需两步操作。
4. **命名空间折叠**：`hicc_usages::xxx::Foo` 在 Rust 端用短名 `Foo`，但 `#[cpp(...)]` 属性中保留完整 C++ 限定名（hicc-build 生成的 wrapper 需要完整类型名）。
5. **重载处理**：C++ 同名函数自动加后缀（如 `add_int_int` / `add_double_double`），因为 Rust 不允许重名。
6. **filter 内置跳过规则**：私有方法、operator、模板本体、STL 参数（例外：`std::size_t` / `std::ptrdiff_t` 已映射为 `usize` / `isize`）、类指针参数、同名重载。

## hicc 核心模式（3 行速览）

```rust
// 1. 引入 C++ 头文件
hicc::cpp! { #include "hicc_usages/my_header.h" }

// 2. 导入 C++ 类（含 factory + destroy 桥接）
hicc::import_class! {
    #[cpp(class = "hicc_usages::my::Counter", destroy = "hicc_usages::my::Counter::free")]
    pub class Counter {
        #[cpp(method = "int get() const")]
        pub fn get(&self) -> i32;
    }
}

// 3. 导入自由函数 + factory（static 方法）
hicc::import_lib! {
    #![link_name = "hicc_usage_my_header_adapter"]
    pub class Counter;
    #[cpp(func = "Counter * hicc_usages::my::Counter::create()")]
    pub fn counter_new() -> Counter;
}
```

## 工具链依赖

- **C++ 端**：`clang++ ≥ 14`（AST 提取）+ 任意 C++17 编译器（g++/clang++/MSVC）
- **Rust 端**：`rustc ≥ 1.75`，依赖通过 hicc 自动拉取（`cc`, `syn`, `quote`, `hicc_autogen`）
- **Python**：≥ 3.8 + `jinja2` + `pyyaml`（rust-gen 工具用）
- **构建系统**：`cmake ≥ 3.16` + `make`（任一即可，build.rs 优先 cmake）

## 跨平台支持

| 平台 | C++ 编译 | Rust 编译 | 链接的 C++ 标准库 | 验证状态 |
|------|----------|-----------|-------------------|----------|
| Linux | g++ / clang++ | ✅ | `libstdc++` | **全部 48/48 通过** |
| macOS | clang++ | ✅（代码就位） | `libc++` | 待验证（仅链接库差异） |
| Windows | MSVC | ✅（代码就位） | MSVC runtime（默认） | 待验证（build.rs 已加 `#[cfg]` 分支） |

`build.rs` 中通过 `#[cfg(target_os = ...)]` 自动切换链接库，无需用户介入。

## 已知限制

1. **类指针参数**：参数为 `T*`（T 为类类型）的方法会被 filter.py 跳过（hicc-build 类型匹配失败）。C++ 端需要用基本类型重写签名。
2. **std::string 返回值**：必须用 `import_class!` 单独处理，不能用类型别名。
3. **AST 大小**：每个特性的 `ast.json` 约 30-90MB（含展开的 STL）。symfilter.py 会早剪枝系统命名空间，但磁盘占用需注意（已加 .gitignore）。
4. **首次 cargo build 较慢**：build.rs 调 cmake 编译 C++ 库，第一次约 5-10 秒；之后增量构建。
