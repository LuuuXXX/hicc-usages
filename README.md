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

**2 种模式 + 4 种 rust_gen 工作机制：**

| 模式 | 机制 | 含义 |
|------|------|------|
| ✅ 直接 | `[attr]` 属性 | rust_gen 直接生成 `#[cpp(class="...")]` / `#[cpp(method="...")]` / `#[cpp(func="...")]` 属性，hicc-build 解析后产出 wrapper |
| ✅ 直接 | `[inject]` 注入 | rust_gen 在 `lib.rs` 的 `hicc::cpp!` 块内自动注入活跃 C++ 代码（typedef、namespace 级 factory、固定参数包装等），避开 hicc-build 不能解析的语法 |
| ✅ 直接 | `[skip]` 跳过 | filter.py 内置规则跳过某些符号（基类指针参数、friend、operator 本体等），不生成对应 Rust 绑定 |
| 💬 注释式注入 | `[suggest]` 建议 | rust_gen 在 `lib.rs` 中以注释形式产出建议包装代码；用户取消注释即启用，不取消对 crate 无影响 |

**48 个特性一览（每行末尾的 → 指向 AST 关键符号映射到的 Rust 块）：**

| # | 特性 | 模式 | 机制 | 关键说明 |
|---|------|------|------|---------|
| 001 | hello_world | ✅ 直接 | `[attr]` | function `hello()` → `import_lib!`；class `Greeter` + `create/free` → `import_class!` + `import_lib!` factory |
| 002 | function_overload | ✅ 直接 | `[attr]` | 同名重载 → `import_lib!` 加参数类型后缀（`add_int_int`） |
| 003 | default_args | ✅ 直接 | `[attr]` | 多个具名重载 → `import_lib!` 各自一行 |
| 004 | inline_functions | ✅ 直接 | `[attr]` | inline function → `import_lib!`（inline 对 FFI 透明） |
| 005 | variadic_functions | ✅ 直接 | `[attr]` | C++ 端预固定参数数量（`sum_2/sum_3/sum_4`） → `import_lib!` |
| 006 | class_basic | ✅ 直接 | `[attr]` | class `Counter { get/inc/dec/reset }` → `import_class!` 方法 |
| 007 | class_constructor | ✅ 直接 | `[attr]` | 私有 ctor + `static T* create(args)` → `import_class!` + `import_lib!` factory |
| 008 | class_copy | ✅ 直接 | `[attr]` | `clone()` 命名方法 → `import_class!`（拷贝 ctor 被 filter 跳过） |
| 009 | class_move | ✅ 直接 | `[attr]` | `take_from()` 命名方法 → `import_class!`（移动 ctor 被 filter 跳过） |
| 010 | class_static | ✅ 直接 | `[attr]` | static 方法 → `import_lib!` 自由函数（带类名前缀） |
| 011 | class_const | ✅ 直接 | `[attr]` | const 方法 → `&self`；非 const → `&mut self`（`import_class!`） |
| 012 | class_volatile | ✅ 直接 | `[attr]` | volatile 方法 → `&self`（`import_class!`，Rust 无 volatile） |
| 013 | inheritance_single | ✅ 直接 | `[attr]` | 基类 + 派生类 → 各自独立 `import_class!` |
| 014 | inheritance_multiple | ✅ 直接 | `[attr]` | 多继承：同上，hicc 把继承展平 |
| 015 | virtual_basic | ✅ 直接 | `[attr]` | 虚函数按普通方法 → `import_class!`（vtable 由 C++ 管理） |
| 016 | virtual_pure | ✅ 直接 | `[attr]` | 抽象类无 factory；派生类各自 `import_class!` |
| 017 | virtual_override | ✅ 直接 | `[attr]` | override 按普通方法 → `import_class!` |
| 018 | virtual_diamond | ✅ 直接 | `[attr]` | 虚继承菱形：派生类各自 `import_class!` |
| 019 | operator_overload | 💬 注释式 | `[suggest]` | C++ 保留 `operator+/-/* ==`；rust_gen 在 `lib.rs` 注释中产出 `vec2_add/sub/eq` 命名包装供用户取消注释启用 |
| 020 | friend_function | ✅ 直接 | `[skip]` | filter 跳过含 `T*` 参数的 friend 函数；仅类方法导出到 `import_class!` |
| 021 | explicit_ctor | ✅ 直接 | `[attr]` | `explicit` + 静态 create → `import_class!` + `import_lib!` factory |
| 022 | mutable_member | ✅ 直接 | `[attr]` | mutable 字段对 filter 透明；方法 → `import_class!` |
| 023 | typeid_rtti | ✅ 直接 | `[skip]` | filter 跳过含基类指针参数的方法；`type_name()` 等自由函数 → `import_lib!` |
| 024 | template_function | ✅ 直接 | `[attr]` | 函数模板 `max_of<T>` → `import_lib!` 直接用 `#[cpp(func = "int max_of<int>(int, int)")]` |
| 025 | template_class | ✅ 直接 | `[inject]` | C++ 纯 `template<typename T> class Stack`；rust_gen 在 `hicc::cpp!` 块注入 `using IntStack = Stack<int>` + namespace 级 `create_int_stack/free_int_stack` → `import_class! IntStack` + `import_lib!` factory |
| 026 | template_specialization | ✅ 直接 | `[inject]` | C++ 纯 `template + 特化`；rust_gen 在 `hicc::cpp!` 块注入 `type_name_int()` 包装调用 `TypeInfo<int>::name()` → `import_lib!` |
| 027 | template_instantiation | ✅ 直接 | `[inject]` | 同 025，多类型实例化（int + double） → `import_class! IntContainer/DoubleContainer` |
| 028 | variadic_template | ✅ 直接 | `[inject]` | C++ 纯变参模板；rust_gen 在 `hicc::cpp!` 块注入固定参数包装（`sum_two/sum_three`） → `import_lib!` |
| 029 | unique_ptr | ✅ 直接 | `[skip]` | PImpl 模式：`Impl` 嵌套类通过 `parentDeclContextId` 被 filter 跳过；`Owner` 类方法 → `import_class!` |
| 030 | shared_ptr | ✅ 直接 | `[skip]` | 同 029 |
| 031 | custom_deleter | ✅ 直接 | `[skip]` | 同 029 + 自定义 deleter |
| 032 | placement_new | ✅ 直接 | `[attr]` | `operator new/delete` 跳过；`Buffer` 包装类 → `import_class!` |
| 033 | raii_pattern | ✅ 直接 | `[attr]` | 析构由 `free()` 桥接 → `import_class!` + `import_lib!` factory |
| 034 | vector_basic | ✅ 直接 | `[attr]` | `std::vector<int>` 字段私有；`IntVector` POD 包装类 → `import_class!` |
| 035 | map_basic | ✅ 直接 | `[attr]` | 同 034，`IntMap` 包装类 → `import_class!` |
| 036 | string_basic | ✅ 直接 | `[attr]` | `std::string` 字段私有；用 `const char*` C 接口 → `import_class!`（不用 hicc-std） |
| 037 | array_basic | ✅ 直接 | `[attr]` | 同 034，`FixedArray` 包装类 → `import_class!` |
| 038 | tuple_basic | ✅ 直接 | `[attr]` | 同 034，`Triple` getter/setter → `import_class!` |
| 039 | lambda_basic | ✅ 直接 | `[attr]` | lambda 不出现在 AST 顶层；C++ 端用 lambda + 命名包装 → `import_class!` |
| 040 | std_function | ✅ 直接 | `[attr]` | `std::function` 字段私有；`set_mode/call` 方法 → `import_class!` |
| 041 | functional_bind | ✅ 直接 | `[attr]` | `std::bind` 不在 AST 顶层；`configure/next` → `import_class!` |
| 042 | exception_basic | ✅ 直接 | `[attr]` | try/catch + `thread_local` 错误码；`last_error()` 自由函数 → `import_lib!` |
| 043 | namespace_nested | ✅ 直接 | `[attr]` | 嵌套命名空间 → 折叠为 Rust 短名；`#[cpp(...)]` 保留完整限定名 |
| 044 | enum_class | ✅ 直接 | `[attr]` | enum class 不直接映射；C++ 端 `color_to_int/int_to_color` 转换 → `import_lib!` |
| 045 | union_basic | 💬 注释式 | `[suggest]` | C++ 保留原始 `union Value`；rust_gen 在 `lib.rs` 注释中产出 `ValueBox` 包装类供用户取消注释启用 |
| 046 | constexpr_basic | ✅ 直接 | `[attr]` | constexpr 对 FFI 透明，按普通函数 → `import_lib!` |
| 047 | noexcept_basic | ✅ 直接 | `[attr]` | 自由函数保留 noexcept → `import_lib!`；成员方法移除 noexcept → `import_class!`（C++ 端 1 处调整） |
| 048 | summary | ✅ 直接 | `[attr]` | 综合：enum + unique_ptr + exception + namespace + class → 多个 `import_class!` / `import_lib!` |

**统计：**

| 模式 | 机制 | 数量 |
|------|------|------|
| ✅ 直接 | `[attr]` 属性 | 39 |
| ✅ 直接 | `[inject]` 注入 | 4（025-028） |
| ✅ 直接 | `[skip]` 跳过 | 3（020、029-031 中 029、023；030、031 共享 029 的 skip 逻辑） |
| 💬 注释式 | `[suggest]` 建议 | 2（019、045） |

注：020 friend 用 `[skip]`，023 typeid 同时含 `[skip]`（基类指针参数）和 `[attr]`（剩余方法）；上表按主导机制归类。

## 数据流映射：AST → Rust 拼装

`lib.rs` 由 4 个区域组成，每种 rust_gen 机制产生不同区域的代码：

```
lib.rs 结构：
┌──────────────────────────────────────────────────────────┐
│ hicc::cpp! {                       ← 区域 A：C++ 头/include + 自动注入  │
│     #include "hicc_usages/<feature>.h"                              │
│     [inject] 注入的 typedef + namespace 级 factory（025-028）        │
│     [suggest] 注释形式的 operator/union 包装（019、045）             │
│ }                                                                    │
├──────────────────────────────────────────────────────────┤
│ hicc::import_class! {              ← 区域 B：类 + 成员方法            │
│     #[cpp(class = "ns::Foo", destroy = "ns::Foo::free")]            │
│     pub class Foo {                                                  │
│         #[cpp(method = "...")]  pub fn method(&self) -> i32;  ← [attr] │
│     }                                                                │
│ }                                                                    │
├──────────────────────────────────────────────────────────┤
│ hicc::import_lib! {                ← 区域 C：自由函数 + factory      │
│     #[cpp(func = "...")]       pub fn foo_new() -> Foo;       ← [attr]│
│     [suggest] 注释形式的 operator 包装 Rust 绑定（019）              │
│ }                                                                    │
├──────────────────────────────────────────────────────────┤
│ // 顶层注释块                       ← 区域 D：union 的 import_class!   │
│ // [suggest] 注释形式的 ValueBox 包装类（045）                       │
└──────────────────────────────────────────────────────────┘
```

### AST 节点 → Rust 区域映射

| AST 节点（clang） | rust_gen 处理 | 落到 lib.rs 哪个区域 | 典型特性 |
|------|---------|----------|------|
| `FunctionDecl`（自由函数，非模板） | 直接生成 `#[cpp(func = "...")]` | 区域 C（`import_lib!`） | 001、004、042 |
| `CXXRecordDecl` / `ClassDecl` | 生成 `import_class!` 块 + factory | 区域 B + 区域 C | 006、007、013 |
| `CXXMethodDecl`（成员方法） | 直接生成 `#[cpp(method = "...")]` | 区域 B（`import_class!` 内） | 006、011、015 |
| `CXXMethodDecl`（static） | 转 `#[cpp(func = "...")]` 自由函数 | 区域 C（`import_lib!` 内） | 010 |
| `CXXMethodDecl`（operator*/+-） | filter 跳过本体；生成命名包装注释 | 区域 A 注释 + 区域 C 注释 | 019 |
| `CXXConstructorDecl` / `CXXDestructorDecl` | filter 跳过；用 `create/free` 替代 | 区域 C（factory 函数） | 007、008 |
| `FunctionTemplateDecl` | 函数模板：`#[cpp(func = "name<int>(...)")]` 直接生成 | 区域 C（`import_lib!`） | 024 |
| `FunctionTemplateDecl`（变参） | filter 跳过；rust_gen 注入固定参数包装 | 区域 A（注入 C++）+ 区域 C（Rust 绑定） | 028 |
| `ClassTemplateDecl` | filter 跳过；rust_gen 注入 typedef + factory | 区域 A（注入 C++）+ 区域 B（`import_class!`）+ 区域 C（factory） | 025、027 |
| `ClassTemplateSpecializationDecl` | filter 跳过；rust_gen 注入 namespace 级包装 | 区域 A（注入 C++）+ 区域 C | 026 |
| `RecordDecl`（`is_union=true`） | filter 跳过；生成 ValueBox 注释 | 区域 A 注释 + 区域 D 注释 | 045 |
| `EnumDecl`（enum class） | filter 跳过；C++ 端用 `int` 转换函数 | 区域 C（`import_lib!` 转换函数） | 044 |
| 含 `T*` 参数的函数/方法 | filter 跳过（hicc-build 类型匹配限制） | 不生成 Rust 绑定 | 020、023 |
| `ClassTemplateSpecializationDecl`（嵌套类 PImpl） | 通过 `parentDeclContextId` 跳过 | 不生成 Rust 绑定 | 029、030、031 |

### 4 种机制的端到端示例

**`[attr]` 属性（001 hello_world）**

```cpp
// C++ 端
namespace hicc_usages::hello_world {
    void hello();
    class Greeter { ... static Greeter* create(const char*); ... };
}
```
↓ rust_gen 解析 AST 的 FunctionDecl + CXXMethodDecl
```rust
// Rust lib.rs（区域 B + 区域 C）
hicc::import_class! {
    #[cpp(class = "hicc_usages::hello_world::Greeter",
          destroy = "hicc_usages::hello_world::Greeter::free")]
    pub class Greeter { ... }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_hello_world_adapter"]
    pub class Greeter;
    #[cpp(func = "void hicc_usages::hello_world::hello()")]
    pub fn hello();
    #[cpp(func = "hicc_usages::hello_world::Greeter * hicc_usages::hello_world::Greeter::create(const char*)")]
    pub fn greeter_new(msg: *const i8) -> Greeter;
}
```

**`[inject]` 注入（025 template_class）**

```cpp
// C++ 端：纯模板，无任何特化
template<typename T> class Stack {
public:
    void push(T); T pop(); size_t size() const;
};
```
↓ rust_gen 在 `hicc::cpp!` 块内注入活跃 C++ 包装（hicc-build 无法直接处理 `Stack<int>* Stack<int>::create()`，必须 typedef 别名）
```rust
// Rust lib.rs（区域 A 注入 + 区域 B 新类 + 区域 C factory）
hicc::cpp! {
    #include "hicc_usages/template_class.h"
    namespace hicc_usages::template_class {
        using IntStack = Stack<int>;
        inline IntStack* create_int_stack() { return new IntStack(); }
        inline void free_int_stack(IntStack* self) { delete self; }
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::template_class::IntStack",
          destroy = "hicc_usages::template_class::free_int_stack")]
    pub class IntStack {
        #[cpp(method = "void push(int)")] pub fn push(&mut self, v: i32);
        #[cpp(method = "std::size_t size() const")] pub fn size(&self) -> usize;
    }
}
hicc::import_lib! {
    #[cpp(func = "hicc_usages::template_class::IntStack * hicc_usages::template_class::create_int_stack()")]
    pub fn int_stack_new() -> IntStack;
}
```

**`[skip]` 跳过（020 friend_function）**

```cpp
// C++ 端
class Account {
    int balance_;
public:
    static Account* create(int);
    void deposit(int);
    friend void transfer(Account* from, Account* to, int amount);  // 含 T* 参数
};
```
↓ filter.py 跳过 `transfer`（基类指针参数），其他正常生成
```rust
// Rust lib.rs（区域 B + 区域 C，注意 friend 函数被跳过）
hicc::import_class! { ... pub class Account { pub fn deposit(&mut self, amount: i32); } }
hicc::import_lib! { ... pub fn account_new(initial: i32) -> Account; }
// transfer 函数未生成（hicc-build 不支持 T* 参数）
```

**`[suggest]` 注释式建议（019 operator_overload）**

```cpp
// C++ 端：保留原始 operator
class Vec2 {
public:
    int x_, y_;
    Vec2 operator+(const Vec2&) const;
    Vec2 operator*(int s) const;
    bool operator==(const Vec2&) const;
};
```
↓ rust_gen 检测到 operator 方法，生成注释式命名包装（filter 跳过 operator 本体）
```rust
// Rust lib.rs（区域 A 注释 + 区域 C 注释，全部以 // 开头）
hicc::cpp! {
    #include "hicc_usages/operator_overload.h"
    // inline Vec2* vec2_add(const Vec2& a, const Vec2& b) { return new Vec2(a + b); }
    // inline Vec2* vec2_mul(const Vec2& a, int s) { return new Vec2(a * s); }
    // inline bool vec2_eq(const Vec2& a, const Vec2& b) { return a == b; }
}
hicc::import_lib! {
    ...
    // #[cpp(func = "ns::Vec2 * ns::vec2_add(const ns::Vec2&, const ns::Vec2&)")]
    // pub fn vec2_add(a: &Vec2, b: &Vec2) -> Vec2;
    // ... 其他 operator 包装也以注释形式列出
}
```
用户取消注释即启用，不取消对 crate 无影响。

### special.yaml 中的机制配置

`[inject]` 机制需要在 special.yaml 显式配置：

```yaml
# 类模板实例化（[inject]）
template_class:
  class_template_instantiations:
    Stack: [int]            # 实例化 IntStack

# 类模板静态方法（[inject]）
template_specialization:
  template_static_wrappers:
    TypeInfo:
      - inst: int
        methods: {name: type_name_int, size_of: size_of_int}

# 变参模板固定参数包装（[inject]）
variadic_template:
  variadic_wrappers:
    sum_all:
      param_type: int
      arities: [{arity: 2, name: sum_two}, {arity: 3, name: sum_three}]
```

其他三种机制（`[attr]` / `[skip]` / `[suggest]`）无需 special.yaml 配置，rust_gen 自动检测。

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
