# hicc-usages — C++ 特性 × hicc FFI 映射示例集

本仓库基于 [`hicc`](./hicc) 框架，把 48 个常见 C++ 特性逐一映射为 Rust Safe FFI。每个示例都包含完整的 C++ 项目（支持 standalone / make / cmake 三种构建）+ 宏展开后的 AST + 手动从 AST 生成的 hicc Rust crate。

> 详细方案见 [`openspec/changes/cpp-feature-matrix/`](./openspec/changes/cpp-feature-matrix/)。

## 目录结构

```
hicc-usages/
├── hicc/                       # hicc 框架（只读子模块）
├── docs/
│   ├── hicc-capabilities.md    # hicc 能力地图（9 个子项目）
│   └── feature-matrix-draft.md # 48 特性 × hicc 模式初判
├── examples/{NNN_name}/        # 一个 C++ 特性 = 一个目录
│   ├── cpp/                    # 独立 C++ 项目（standalone/make/cmake）
│   ├── ast/
│   │   └── user-ast.json       # 过滤后的精简 AST（仅用户头文件节点）
│   ├── rust_hicc/              # 基于 hicc 的 Rust crate
│   └── ast-to-hicc-notes.md    # 手动 AST→hicc 方案记录
├── tools/
│   ├── dump_ast.sh             # clang -E + clang -ast-dump 包装
│   └── filter_ast.py           # 把全量 AST 过滤为用户节点
└── openspec/                   # OpenSpec SDD 方案文档
```

## 表 1：C++ 特性 × hicc 支持方式（汇总）

> ✅ 直接支持 | ⚠️ 部分支持（需 cpp! 块包装）| ❌ 不支持

| # | 特性 | hicc 支持 | hicc 模式 | 关键代码片段 |
|---|---|---|---|---|
| 001 | hello_world | ✅ | `import_lib!` + `#[cpp(func)]` | `#[cpp(func = "void ns::hello()")] fn hello();` |
| 002 | function_overload | ✅ | `import_lib!` 多个 fn（Rust 改名） | `add_int` / `add_double` / `add_string` |
| 003 | default_args | ✅ | `#[cpp(func = "...")]` 写全签名，Rust 省略尾部 | `fn greet_default(name: &string) -> i32;` |
| 004 | inline_functions | ✅ | 透明（与普通函数同） | 同 001 |
| 005 | variadic_functions | ✅ | `unsafe fn f(arg0, ...)`，调用 `f()(args...)` | `unsafe { sum_ints()(3, 10, 20, 30) }` |
| 006 | class_basic | ✅ | `import_class!` + 工厂 | `class Counter { fn inc(&mut self); }` |
| 007 | class_constructor | ✅ | 每个 ctor 一个 `make_unique` 工厂 | `make_unique<T, int>(int&&)` |
| 008 | class_copy | ✅ | copy/move ctor 工厂 | `make_unique<T, T&&>(T&&)` for move |
| 009 | class_move | ✅ | move ctor 工厂 + cpp! 包装 operator+= | `holder_move(other: Self) -> Self` |
| 010 | class_static | ✅ | 静态方法/字段用 cpp! 命名空间级包装 | `inline int class_static_alive() { return Counter::alive(); }` |
| 011 | class_const | ✅ | `#[cpp(method = "...const")]` 保留 const | `#[cpp(method = "float value() const")]` |
| 012 | class_volatile | ⚠️ | C++ 端必须提供非 volatile 桥接 | Rust 只绑定 `safe_read()`/`safe_write()` |
| 013 | inheritance_single | ✅ | 派生类独立 import_class!；基类方法借继承 | `#[cpp(class = "ns::Dog")] pub class Dog` |
| 014 | inheritance_multiple | ⚠️ | 多继承不支持 #[interface]，只绑派生类 | 不暴露 Drawable*/Serializable* |
| 015 | virtual_basic | ✅ | 派生类绑定，C++ vtable 自动 dispatch | `#[cpp(method = "float area() const")]` |
| 016 | virtual_pure | ✅ | 抽象基类不实例化，只绑具体派生类 | 只暴露 InMemoryStorage |
| 017 | virtual_override | ✅ | 派生类独立绑定，vtable 自动 dispatch | `#[cpp(class = "ns::Triangle")] pub class Triangle` |
| 018 | virtual_diamond | ⚠️ | 菱形虚继承：所有方法必须 cpp! 包装 | `inline int diamond_read(IOCombo& c)` |
| 019 | operator_overload | ⚠️ | operator 重载必须 cpp! 块命名空间包装 | `inline Vec2 vec_add(const Vec2&, const Vec2&)` |
| 020 | friend_function | ✅ | friend 在 Rust 侧透明 | `#[cpp(func = "long ns::audit_total(const ns::Account&)")]` |
| 021 | explicit_ctor | ✅ | explicit 对 FFI 透明；每 ctor 一个工厂 | `make_unique<T, Args&&...>(Args&&...)` |
| 022 | mutable_member | ✅ | mutable 在 const 方法中可改，FFI 透明 | `#[cpp(method = "T name() const")] &self` |
| 023 | typeid_rtti | ⚠️ | typeid 必须包装（type_info 非 FFI 友好） | `const char* type_name_base(const Base&)` 包装 |
| 024 | template_function | ⚠️ | 显式实例化 + cpp! 包装转 by value | `inline int add_int_wrap(int, int) { return add<int>(...) }` |
| 025 | template_class | ⚠️ | using 别名 + cpp! 包装 const T&（原始类型） | `using StackInt = ns::Stack<int>;` + `inline void stack_int_push_wrap(...)` |
| 026 | template_specialization | ✅ | 每个特化暴露自由函数 | `#[cpp(func = "const char* ns::int_name()")] pub fn int_name() -> *const i8;` |
| 027 | template_instantiation | ✅ | using 别名 + make_unique 工厂 | `using PairInt = ns::Pair<int>;` + `make_unique<PairInt, int, int>(int&&, int&&)` |
| 028 | variadic_template | ⚠️ | 每个 arity 一个具现化包装函数 | `inline int sum_two(int,int){ return sum<int,int>(...); }` |
| 029 | unique_ptr | ✅ | 默认 deleter → 当对象本身，Drop = 释放 | `make_unique<T, int, const std::string&>(int&&, const std::string&)` |
| 030 | shared_ptr | ✅ | `hicc::shared_ptr<T>` Rust 类型 + `.get()` | `pub fn make_counter(start: i32) -> hicc::shared_ptr<Counter>;` |
| 031 | custom_deleter | ⚠️ | opaque handle 包装（避开数组/自定义 deleter） | `IntArrayHandle` struct + `void*` cpp! 包装 |
| 032 | placement_new | ⚠️ | void* 屏蔽 Payload* + Rust 端 RAII | `inline void* cd_place(void*, int) { return place_payload_raw(...); }` |
| 033 | raii_pattern | ✅ | `import_class!` + make_unique 工厂；Rust Drop = C++ dtor | `make_unique<FileHandle, int, const std::string&>(int&&, const std::string&)` |
| 034 | vector_basic | ✅ | `hicc_std::vector<Pod<T>>` 别名 + make_unique 工厂 | `class RustVec = hicc_std::vector<hicc::Pod<i32>>;` |
| 035 | map_basic | ✅ | `hicc_std::map<Pod<K>, V>` 别名 + make_unique 工厂 | `class RustMap = hicc_std::map<hicc::Pod<i32>, hicc_std::string>;` |
| 036 | string_basic | ✅ | 直接绑业务函数（std::string 内置实例化） | `pub fn greet(name: &hicc_std::string) -> hicc_std::string;` |
| 037 | array_basic | ✅ | `hicc_std::array<Pod<T>>` 单参数别名，N 在工厂中实例化 | `make_unique<std::array<int, 5>>()` |
| 038 | tuple_basic | ⚠️ | 不能直接 FFI，cpp! 包装字段访问器 | `inline int triple_id(const Triple& t) { return std::get<0>(t); }` |
| 039 | lambda_basic | ✅ | `std::function<R(Args...)>` → `hicc::Function<fn(Args...) -> R>` | `pub fn apply_int(x: i32, fn_: hicc::Function<fn(i32) -> i32>) -> i32;` |
| 040 | std_function | ✅ | 同 039，std::function 作为类成员 | `pub fn replace(&mut self, fn_: hicc::Function<fn(i32) -> i32>);` |
| 041 | functional_bind | ✅ | `std::bind` 透明 — 包成 `std::function` 后映射 `hicc::Function` | `pub fn make_adder(n: i32) -> hicc::Function<fn(i32) -> i32>;` |
| 042 | exception_basic | ✅ | `hicc::Exception<T>` 返回类型；hicc-build 自动 try/catch | `pub fn safe_divide(a: i32, b: i32) -> hicc::Exception<i32>;` |
| 043 | namespace_nested | ✅ | 嵌套命名空间完全透明 | `#[cpp(class = "n1::n2::n3::Foo")] pub class Foo` |
| 044 | enum_class | ⚠️ | int↔enum 转换器 + Rust `#[repr(i32)] enum` 镜像 | `inline int color_to_int(Color c) { return (int)c; }` |
| 045 | union_basic | ⚠️ | POD union 用 `#[repr(C)] struct([u8; N])` 镜像 + 类型化 accessor | `value_as_int(v: Value) -> i32; make_value_int(x: i32) -> Value;` |
| 046 | constexpr_basic | ✅ | `#[cpp(data = "ns::Class::FIELD")]` → `&'static T` | `#[cpp(data = "ns::Constants::PI")] pub fn pi() -> &'static f64;` |
| 047 | noexcept_basic | ✅ | noexcept 完全透明（hicc-build 不感知） | 与 001 相同 |
| 048 | summary | ✅ | 综合示例：Exception + enum + 单继承 + constexpr + vector | 见 `examples/048_summary/` |

> **进度**：批次 A-F (001-048) 全部完成 — 48 个 crate 共 100+ 个 smoke test 通过。

## 表 2：手动处理方案 × 自动化可行性

| # | 特性 | 手动方案要点 | 从 AST 提取的关键信息 | 自动化可行性 |
|---|---|---|---|---|
| 001 | hello_world | 1 个 `import_lib!` 函数 | `FunctionDecl.name` + `type.qualType` | **高** |
| 002 | function_overload | 同名 C++ 函数 → 多个 Rust 函数（按参数类型改名） | `FunctionDecl.type.qualType` 全签名 | **高**（type-based mangling） |
| 003 | default_args | 每个 arity 一个 Rust 函数；`#[cpp(func)]` 写完整签名 | `ParmVarDecl` 是否带 `defaultArg` | **中**（笛卡尔展开） |
| 004 | inline_functions | 与普通函数同 | 同 001 | **高** |
| 005 | variadic_functions | 用 `unsafe fn(a, ...)` 模式；调用方式特殊 | `ParmVarDecl.type.qualType` 末尾 `...` | **中**（需文档说明调用） |
| 006 | class_basic | `import_class!` + `make_unique` 工厂 + Rust `new` 包装 | `CXXRecordDecl` + `CXXMethodDecl` + `CXXConstructorDecl` | **中-高** |
| 007 | class_constructor | 多 ctor → 多 `make_unique<T, Args...>` 工厂 | `CXXConstructorDecl.type.qualType` 各参数类型 | **高**（机械） |
| 008 | class_copy | copy/move ctor 模板固定（`const T&` / `T&&`） | `CXXConstructorDecl` 是否 `&&` | **中** |
| 009 | class_move | move 工厂 + operator 重载包装 | `CXXMethodDecl.name` 含 `operator` 前缀 | **高**（机械） |
| 010 | class_static | 静态方法/字段 → cpp! 块的 inline 包装 | `CXXMethodDecl.isStatic` + `VarDecl.isStatic` | **高** |
| 011 | class_const | const/非 const 方法 → `&self` / `&mut self` | `CXXMethodDecl.isConst` | **高** |
| 012 | class_volatile | 必须有 C++ 桥接；Rust 只绑桥接 | `CXXMethodDecl.isVolatile` | **中**（需 C++ 改动） |
| 013 | inheritance_single | 派生类独立绑定，基类继承透明 | `CXXRecordDecl.bases` + `methods.isVirtual` | **高** |
| 014 | inheritance_multiple | 只绑派生类，多基类不暴露 | `bases.length > 1` | **中**（递归扫基类 method） |
| 015 | virtual_basic | 派生类绑定，vtable 自动 | `methods.isOverride` | **高** |
| 016 | virtual_pure | 跳过抽象基类（`isAbstract`），只绑派生 | `CXXRecordDecl.isAbstract` | **高** |
| 017 | virtual_override | 派生类独立绑定，override 透明 | `CXXMethodDecl.isOverride` | **高** |
| 018 | virtual_diamond | 强制 cpp! 包装所有方法（pointer-to-member 报错） | `CXXRecordDecl.bases[*].isVirtual` | **中**（必须包装所有方法） |
| 019 | operator_overload | operator 前缀方法名 → 包装函数 | `CXXMethodDecl.name` 含 `operator` | **高**（机械映射） |
| 020 | friend_function | 普通自由函数处理 | `FunctionDecl` 在 namespace 顶层 | **高** |
| 021 | explicit_ctor | explicit 透明，每 ctor 一个工厂 | `CXXConstructorDecl` 数量 | **高** |
| 022 | mutable_member | mutable 透明，const 方法用 &self | `FieldDecl.isMutable`（不需判定） | **高** |
| 023 | typeid_rtti | typeid 必须包装（type_info） | 自由函数含 typeid 表达式 | **中**（需 C++ 改动） |
| 024 | template_function | 模板显式实例化 + cpp! 包装 by value | `FunctionTemplateDecl` + 显式实例化列表 | **中**（需 by value 包装） |
| 025 | template_class | using 别名 + const T& 对原始类型需包装 | `ClassTemplateDecl` + `CXXMethodDecl` 参数类型 | **中-高** |
| 026 | template_specialization | 每个特化暴露自由函数 | `ClassTemplateSpecializationDecl` + 对应 FunctionDecl | **高** |
| 027 | template_instantiation | using 别名 + make_unique 工厂 | 显式实例化（ExternTemplate/ClassTemplateSpecialization） | **高** |
| 028 | variadic_template | 每个 arity 一个具现化包装 | `FunctionTemplateDecl` + 调用点 arity | **中** |
| 029 | unique_ptr | 默认 deleter → 类对象本身 | `ClassTemplateSpecializationDecl` primary=unique_ptr，deleter=默认 | **高** |
| 030 | shared_ptr | `hicc::shared_ptr<T>` Rust 类型 | `ClassTemplateSpecializationDecl` primary=shared_ptr | **高** |
| 031 | custom_deleter | opaque handle struct + void* cpp! 包装 | unique_ptr 模板参数非默认 deleter 或数组 | **中**（需 C++ 改动） |
| 032 | placement_new | void* 屏蔽 + Rust 端 RAII 包装 | `CXXNewExpr` 带 placement arg + `~T()` 显式析构 | **低**（需识别语义模式） |
| 033 | raii_pattern | 复用 029 模式：make_unique 工厂 + Rust Drop | 工厂返回 unique_ptr<T>，方法在类上 | **高** |
| 034 | vector_basic | typedef + hicc_std::vector<Pod<T>> 别名 + 工厂 | `ClassTemplateSpecializationDecl` primary=vector | **高** |
| 035 | map_basic | typedef + hicc_std::map<Pod<K>, V> 别名 + 工厂 | `ClassTemplateSpecializationDecl` primary=map | **高** |
| 036 | string_basic | 直接绑业务函数（hicc-std 内置） | `ClassTemplateSpecializationDecl` primary=basic_string | **高** |
| 037 | array_basic | typedef + 单参数 array<Pod<T>>，N 在工厂实例化 | `ClassTemplateSpecializationDecl` primary=array（N 是模板非类型参数） | **高** |
| 038 | tuple_basic | 字段访问器自由函数 + Triple 不透明类 | `ClassTemplateSpecializationDecl` primary=tuple，无 hicc-std 内置 | **中**（需为每字段写 wrapper） |
| 039 | lambda_basic | std::function → hicc::Function | `FunctionDecl` 含 `std::function<R(Args...)>` 参数/返回 | **高** |
| 040 | std_function | 同 039；std::function 作为类成员 | `FieldDecl` 类型为 `std::function<...>` | **高** |
| 041 | functional_bind | std::bind 透明，包成 std::function 后映射 | `FunctionDecl` 返回/参数含 `std::function<R(Args...)>` | **高** |
| 042 | exception_basic | `hicc::Exception<T>` 返回类型；hicc-build 自动 try/catch | `FunctionDecl` 是否含 `throw` / `CallExpr` 调用抛异常的函数（语义判断） | **中**（需开发者显式标 Exception） |
| 043 | namespace_nested | 完整带命名空间签名 | `qualifiedName` 字段直接用 | **高** |
| 044 | enum_class | C++ int↔enum 转换器 + Rust 镜像 enum + inline 包装 | `EnumDecl` scoped=true | **中**（每方法需 inline 包装） |
| 045 | union_basic | POD union 用 `#[repr(C)] struct([u8; N])` 镜像 + 类型化 accessor | `RecordDecl` kind=Union，需判定 isTriviallyCopyable | **中**（非平凡需 unique_ptr 包装） |
| 046 | constexpr_basic | `#[cpp(data = "...")]` 暴露为 `&'static T` | `VarDecl::isConstexpr` + `FunctionDecl::isConstexpr` | **高** |
| 047 | noexcept_basic | 完全透明 — 不需任何处理 | `FunctionDecl::exceptionSpecType == EST_BasicNoexcept`（仅文档用） | **高**（自动忽略） |
| 048 | summary | 综合示例，混合 5+ 种模式 | 多种 AST 节点组合 | **低**（需多种模式组合判断） |

### 自动化可行性分级标准

- **高**：AST 字段→hicc 输出有 1:1 映射，零业务判断
- **中**：需要决策（命名、arity 组合、参数传递方式），但模板可枚举
- **低**：需要语义理解（如 RTTI/operator 重载的 cpp! 包装怎么写）

## 踩坑总览（48 示例实战经验）

| 坑 | 现象 | 解决 |
|---|---|---|
| std::string 参数 | cc-rs 编译报 incomplete type | `hicc::cpp!` 块内 `#include <hicc/std/string.hpp>` |
| `&T` 返回值 | 调用方法时 SIGSEGV | `import_class!` 顶部 `class string = hicc_std::string;` 启用 ClassRef 自动转换 |
| `make_unique<T, Args>` 模板 | "no matches converting function" | Args 类型要匹配 ctor 实参；按值参数用 `T&&`（forwarding ref） |
| 缺省参数 | C++ 函数指针类型不匹配 | `#[cpp(func = ...)]` 始终写**完整**签名，Rust fn 省略 |
| 变长参数 | "argument count mismatch" | hicc 把 `unsafe fn(a, ...)` 编译成 `fn() -> fn(...)`，调用方式 `f()(args...)` |
| unique_ptr<T[]> / 自定义 deleter | hicc 当类指针注册时 incomplete type | C++ 侧加 opaque handle struct，Rust 端 cpp! 块用 `void*` 屏蔽 |
| make_unique<T, ...> 模板不完整 | `AbiClassMethods<T, unique_ptr<T>>::class_methods incomplete` | 在 cpp! 块用 `using T_alias = ns::T;` 起别名，import_class 与工厂都用别名 |
| const 方法 vs `&self` 不匹配 | hicc-build `check_func_with_rust` 严格校验 const 对齐 | C++ 方法不加 const 就别在 Rust 端用 `&self`（改 `&mut self`，或给 C++ 方法加 const） |
| enum class 不能直接 FFI | hicc 无 scoped enum 表达 | C++ 写 int↔enum 转换器 + Rust 端 `#[repr(i32)] enum` 镜像（044） |
| POD union 按值传 | union 非 class，hicc 当 opaque | Rust 用 `#[repr(C)] struct([u8; N])` 镜像字节块，C++ 写类型化 accessor（045） |
| Exception 跨 FFI | 函数可能抛异常导致 UB | Rust 端用 `hicc::Exception<T>` 返回类型，hicc-build 自动 try/catch（042） |
| 抽象基类 `Base&` 参数 | hicc 无多态 Base 类型 | cpp! 块 inline 包装转派生类引用（048 借鉴 019 operator 模式） |

## 如何运行单个示例

```bash
# C++ 独立构建 + 运行
cd examples/006_class_basic/cpp
bash standalone.sh           # g++ 直接构建
make                         # 或 make
mkdir build && cd build && cmake .. && make  # 或 cmake

# Rust 构建 + 测试
cd examples/006_class_basic/rust_hicc
cargo test

# 重新生成 AST
cd /root/cpp2rust/hicc-usages
bash tools/dump_ast.sh examples/006_class_basic/cpp class_basic
```

## 如何重跑某一批

每批 ≤ 8 项，串行执行避免内存爆掉：

```bash
# 批次 A (001-008)
for n in 001_hello_world 002_function_overload 003_default_args \
         004_inline_functions 005_variadic_functions \
         006_class_basic 007_class_constructor 008_class_copy; do
  (cd examples/$n/cpp && bash standalone.sh) || exit 1
  (cd examples/$n/rust_hicc && cargo test) || exit 1
done

# 批末清理（释放磁盘与 target/ 缓存）
find examples/ -name target -type d -exec rm -rf {} + 2>/dev/null
find examples/ -name build -type d -exec rm -rf {} + 2>/dev/null
```

## 已知限制

- 仅 Linux + g++/clang + stdc++ 验证（Windows/MSVC 未覆盖）
- AST 全量 `ast.json` 约 130 MB（含 std 头），`.gitignore` 排除；仅保留过滤后的 `user-ast.json`（约 2-5 KB）
- 047_noexcept 实测：noexcept 对 hicc-build 完全透明，不需要 C++ 侧修改（旧 memory 已修正）

## 进度看板

| Phase | 状态 | 说明 |
|---|---|---|
| P1: hicc 能力地图 | ✅ | `docs/hicc-capabilities.md` |
| P2: 特性矩阵设计 | ✅ | `docs/feature-matrix-draft.md` + 6 批分组 |
| P3 批 A: C++ 项目 | ✅ | 001-008 standalone/make/cmake 全通过 |
| P4 批 A: AST 导出 | ✅ | 8 个 `user-ast.json` |
| P5 批 A: Rust crate | ✅ | 8 个 crate cargo test 通过（22 个 smoke test） |
| P3 批 B: C++ 项目 | ✅ | 009-016 standalone/make/cmake 全通过 |
| P4 批 B: AST 导出 | ✅ | 8 个 `user-ast.json` |
| P5 批 B: Rust crate | ✅ | 8 个 crate cargo test 通过（18 个 smoke test） |
| P3 批 C: C++ 项目 | ✅ | 017-024 standalone/make/cmake 全通过 |
| P4 批 C: AST 导出 | ✅ | 8 个 `user-ast.json` |
| P5 批 C: Rust crate | ✅ | 8 个 crate cargo test 通过（18 个 smoke test） |
| P3 批 D: C++ 项目 | ✅ | 025-032 standalone/make/cmake 全通过 |
| P4 批 D: AST 导出 | ✅ | 8 个 `user-ast.json` |
| P5 批 D: Rust crate | ✅ | 8 个 crate cargo test 通过（17 个 smoke test） |
| P3 批 E: C++ 项目 | ✅ | 033-040 standalone/make/cmake 全通过 |
| P4 批 E: AST 导出 | ✅ | 8 个 `user-ast.json` |
| P5 批 E: Rust crate | ✅ | 8 个 crate cargo test 通过（16 个 smoke test） |
| P3 批 F: C++ 项目 | ✅ | 041-048 standalone/make/cmake 全通过 |
| P4 批 F: AST 导出 | ✅ | 8 个 `user-ast.json` |
| P5 批 F: Rust crate | ✅ | 8 个 crate cargo test 通过（28 个 smoke test） |
| P6: README 矩阵 | ✅ | 本文件，48 行全量；合计 119 个 smoke test |
