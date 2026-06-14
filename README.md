# hicc-usages — C++ 特性 → Rust FFI 使用手册

48 个独立的 C++ 特性项目，每个都通过 [hicc](./hicc/) 安全 FFI 框架手工转换为等价的 Rust crate。每个特性目录是一个**自包含的、可独立编译运行的最小项目**，并附带：

- 纯 C++ API 源码（`.h` / `.cpp`）+ C++ demo（`main.cpp`）
- 三套 C++ 构建（standalone / Make / CMake），产出同一 `lib<name>.a`
- 宏展开后的 clang AST（`ast/ast.json` + `ast/preprocessed.cpp`）
- 基于 hicc 的 Rust FFI 项目（`rust_hicc/`，含 `lib.rs` + `build.rs` + `tests/smoke.rs`）
- README 记录手动映射步骤、关键 AST 字段、自动化可行性

> **hicc 是只读子模块**，本项目从不修改 hicc 源码。整个项目仅 047 一处例外（C++ 端成员方法移除 `noexcept`，hicc-build 类型匹配限制所致）。

## 与 cpp2rust-demo 的差异

| 维度 | cpp2rust-demo | hicc-usages（本项目）|
|------|---------------|----------------------|
| FFI 机制 | `extern "C"` + opaque pointer + 手写 C wrapper | hicc `#[cpp(...)]` 安全 FFI |
| C++ 端调整 | 每个 .h 必须 `extern "C"` 包裹 | 保持纯 C++ API（仅 047 一处例外）|
| 类型安全 | 全部裸指针 | hicc 编译期类型校验 |
| 异常 | 不支持跨 FFI | `hicc::Exception<T>` |
| STL 容器 | 手写包装 | hicc-std 直接支持 |
| AST 工具 | 无 | 每个特性产出 AST JSON |

## 快速开始

```bash
# 单特性端到端（cpp 三套构建 + AST + cargo build + cargo test）
./scripts/verify-one.sh 001

# 批量构建 C++ 项目（三种构建系统）
./scripts/cpp-build-all.sh

# 批量跑 Rust 冒烟测试
./scripts/rust-test-all.sh
```

依赖：
- C++ 编译器（g++ ≥ 9 或 clang++ ≥ 10）
- clang++ ≥ 10（用于 AST 提取）
- Rust（stable）+ Cargo

## 目录结构

```
hicc-usages/
├── README.md                          # 本文件
├── docs/hicc-capabilities.md          # hicc 能力图谱（9 子 crate + C++ 构造支持表）
├── tools/
│   ├── ast-extract/extract.sh         # AST 提取脚本
│   ├── cpp-templates/                 # C++ 构建模板
│   └── scaffold-feature.sh            # 新特性脚手架
├── scripts/
│   ├── verify-one.sh                  # 单特性端到端
│   ├── cpp-build-all.sh
│   └── rust-test-all.sh
├── hicc/                              # 只读子模块
└── examples/
    ├── 001_hello_world/
    │   ├── README.md                  # 特性说明
    │   ├── cpp/                       # C++ 项目（3 套构建）
    │   ├── ast/                       # AST 产物
    │   └── rust_hicc/                 # Rust FFI 项目
    ├── 002_function_overload/
    │   └── ...
    └── ... (共 48 个)
```

## 48 特性对照表

每列含义：
- **hicc 支持度**：✅ 直接 / 💬 注释式注入 / ⚠️ 需 C++ 端调整 / ❌ 不支持
- **关键 AST 字段**：转译时从 clang AST 提取的字段
- **手工映射**：核心步骤（详见各 README）
- **自动化**：高 / 中 / 低（hicc rust_gen 自动化的难度）

### 第一部分：基础（001-005）

| 编号 | 特性 | 作用 | hicc 支持度 | 关键 AST 字段 | 手工映射 | 自动化 |
|------|------|------|-------------|---------------|----------|--------|
| 001 | hello_world | 最简自由函数 | ✅ 直接 | `FunctionDecl.name/returnType/parameters` | `#[cpp(func="ret name(args)")]` | 高 |
| 002 | function_overload | 函数重载 | ✅ 直接 | `FunctionDecl` ×N（同名）| Rust 加类型后缀 | 高 |
| 003 | default_args | 默认参数 | ✅ 直接 | `ParmVarDecl.defaultArg` | Rust 补完整签名 | 高 |
| 004 | inline_functions | inline 函数 | ✅ 直接 | `FunctionDecl.isInline`（透明）| 普通函数路径 | 高 |
| 005 | variadic_functions | 可变参数 `...` | ⚠️ C++ 端需调整 | `FunctionDecl.isVariadic` | C++ 端写固定 arity 包装 | 中 |

### 第二部分：类基础（006-012）

| 编号 | 特性 | 作用 | hicc 支持度 | 关键 AST 字段 | 手工映射 | 自动化 |
|------|------|------|-------------|---------------|----------|--------|
| 006 | class_basic | 基础类 + factory | ✅ 直接 | `CXXRecordDecl` + 静态 create/free | `import_class!` + `destroy="..."` | 高 |
| 007 | class_constructor | 有参构造 | ✅ 直接（factory） | `CXXConstructorDecl` | `static T* create(args)` + free | 高 |
| 008 | class_copy | 拷贝构造 | ✅ 直接 | `CXXConstructorDecl.isCopy` | clone 包装函数 | 高 |
| 009 | class_move | 移动构造 | ✅ 直接 | `CXXConstructorDecl.isMove` | `T&&` 方法用 self 接收 | 中 |
| 010 | class_static | 静态成员 | ✅ 直接 | `CXXMethodDecl.isStatic` | 在 `import_lib!` 中描述 | 高 |
| 011 | class_const | const 成员 | ✅ 直接 | `CXXMethodDecl.isConst` | `#[cpp(method="ret f() const")]` | 高 |
| 012 | class_volatile | volatile 成员 | ✅ 直接 | `CXXMethodDecl.isVolatile` | `#[cpp(method="ret f() volatile")]` | 高 |

### 第三部分：继承与多态（013-018）

| 编号 | 特性 | 作用 | hicc 支持度 | 关键 AST 字段 | 手工映射 | 自动化 |
|------|------|------|-------------|---------------|----------|--------|
| 013 | inheritance_single | 单继承 | ✅ 直接 | `CXXRecordDecl.bases` | 派生类独立 `import_class!`，合并基类公共方法 | 中 |
| 014 | inheritance_multiple | 多继承 | ✅ 直接（或 interface） | `CXXRecordDecl.bases` ×N | 同上 或 `#[interface]`+`@make_proxy` | 中 |
| 015 | virtual_basic | 虚函数 | ✅ 直接 | `CXXMethodDecl.isVirtual` | 普通方法路径 | 中 |
| 016 | virtual_pure | 纯虚函数 | ✅ 直接 | `CXXMethodDecl.isPure` | 只描述派生类 | 中 |
| 017 | virtual_override | override | ✅ 直接 | `CXXMethodDecl.overriddenMethods` | 同 013 | 中 |
| 018 | virtual_diamond | 菱形虚继承 | ⚠️ 限制标注 | 复杂 vtable | 简化为组合；README 标注 | 低 |

### 第四部分：运算符与特殊成员（019-023）

| 编号 | 特性 | 作用 | hicc 支持度 | 关键 AST 字段 | 手工映射 | 自动化 |
|------|------|------|-------------|---------------|----------|--------|
| 019 | operator_overload | 运算符重载 | 💬 注释式注入 | `CXXMethodDecl` operator+ 等 | C++ 端写命名包装函数（`vec2_add`）| 中 |
| 020 | friend_function | 友元函数 | ✅ 直接 | `FriendDecl`（透明）| 暴露为自由函数 | 高 |
| 021 | explicit_ctor | explicit 构造 | ✅ 直接（factory） | `CXXConstructorDecl.isExplicit` | factory 模式 | 高 |
| 022 | mutable_member | mutable 成员 | ✅ 直接 | `FieldDecl.isMutable`（透明）| 普通 const 方法 | 高 |
| 023 | typeid_rtti | typeid / RTTI | ✅ 直接 | `CXXRecordDecl` 多态 | 命名函数返回类型名/ID | 中 |

### 第五部分：模板（024-028）

| 编号 | 特性 | 作用 | hicc 支持度 | 关键 AST 字段 | 手工映射 | 自动化 |
|------|------|------|-------------|---------------|----------|--------|
| 024 | template_function | 函数模板 | ✅ 直接 | `FunctionTemplateDecl` + 显式实例化 | `#[cpp(func="ret f<T>(args)")]` | 高 |
| 025 | template_class | 类模板 | 💬 活跃注入 typedef+factory | `ClassTemplateDecl` | `hicc::cpp!` 块内 `using FooInt = Foo<int>;` + factory | 中 |
| 026 | template_specialization | 模板特化 | 💬 活跃注入 | `ClassTemplateSpecializationDecl` | namespace 级包装调用特化静态方法 | 中 |
| 027 | template_instantiation | 显式实例化 | 💬 活跃注入 | `ClassTemplateSpecializationDecl` | typedef + factory | 中 |
| 028 | variadic_template | 变参模板 | 💬 活跃注入固定 arity | `FunctionDecl` variadic template | `hicc::cpp!` 块内 sum_two/sum_three | 中 |

### 第六部分：智能指针与内存（029-033）

| 编号 | 特性 | 作用 | hicc 支持度 | 关键 AST 字段 | 手工映射 | 自动化 |
|------|------|------|-------------|---------------|----------|--------|
| 029 | unique_ptr | unique_ptr | ✅ 直接 | `returnType.qualType="std::unique_ptr<T>"` | 返回值类型剥 `unique_ptr<>` | 高 |
| 030 | shared_ptr | shared_ptr | ✅ 直接 | `returnType.qualType="std::shared_ptr<T>"` | 同上 | 高 |
| 031 | custom_deleter | 自定义删除器 | ✅ 直接 | `unique_ptr<T,Deleter>` | `destroy="free_func"` | 中 |
| 032 | placement_new | placement new | ✅ 直接 | `CXXNewExpr.isPlacement` | `T* construct_at(buf, args)` factory | 低 |
| 033 | raii_pattern | RAII 模式 | ✅ 直接 | 析构 + Drop | `destroy="..."` 给 Drop | 高 |

### 第七部分：STL 容器（034-038）

| 编号 | 特性 | 作用 | hicc 支持度 | 关键 AST 字段 | 手工映射 | 自动化 |
|------|------|------|-------------|---------------|----------|--------|
| 034 | vector_basic | std::vector | ✅ 直接（hicc-std） | `parameters[].type.qualType` 含 `std::vector<` | `class Vec = hicc_std::vector<Pod<T>>` | 中 |
| 035 | map_basic | std::map | ✅ 直接（hicc-std） | 同上 | `class Map = hicc_std::map<Pod<K>,Pod<V>>` | 中 |
| 036 | string_basic | std::string | ✅ 直接 | `import_class!` `class string { c_str() }` | **不要**用 `hicc_std::string` alias | 高 |
| 037 | array_basic | std::array | ✅ 直接（hicc-std） | `std::array<T,N>` | typedef `CppArr=std::array<T,N>` | 中 |
| 038 | tuple_basic | std::tuple | ⚠️ 限制标注 | std::tuple | 命名 accessor `first/second` | 低 |

### 第八部分：函数对象（039-041）

| 编号 | 特性 | 作用 | hicc 支持度 | 关键 AST 字段 | 手工映射 | 自动化 |
|------|------|------|-------------|---------------|----------|--------|
| 039 | lambda_basic | lambda | ⚠️ C++ 端需调整 | `LambdaExpr`（内部）| C++ 端命名包装函数 | 低 |
| 040 | std_function | std::function | ⚠️ C++ 端需调整 | `FunctionType` | 命名包装 | 低 |
| 041 | functional_bind | std::bind | ⚠️ 限制标注 | `CXXBindTemporaryExpr` | 命名包装 | 低 |

### 第九部分：异常与高级（042-048）

| 编号 | 特性 | 作用 | hicc 支持度 | 关键 AST 字段 | 手工映射 | 自动化 |
|------|------|------|-------------|---------------|----------|--------|
| 042 | exception_basic | throw 异常 | ✅ 直接 | `FunctionDecl` throws | Rust 返回 `hicc::Exception<T>`，`.ok()` 转 Result | 高 |
| 043 | namespace_nested | 嵌套 namespace | ✅ 直接 | `NamespaceDecl` 嵌套 | Rust 短名 + `#[cpp]` 保留完整限定 | 高 |
| 044 | enum_class | 强类型枚举 | ⚠️ C++ 端需调整 | `EnumDecl` | int 转换函数 `color_to_int/int_to_color` | 中 |
| 045 | union_basic | 共用体 | 💬 注释式注入 | `CXXRecordDecl.isUnion` | ValueBox 包装类（type_tag + from_X/as_X）| 中 |
| 046 | constexpr_basic | constexpr | ✅ 直接（透明）| `FunctionDecl.isConstexpr` | 普通函数路径 | 高 |
| 047 | noexcept_basic | noexcept | ⚠️ C++ 端需调整 | `CXXMethodDecl.isNoexcept` | C++ 端移除成员方法 noexcept（**唯一例外**）| 中 |
| 048 | summary | 综合 | ✅ 直接 | 综合 | 链接上述模式 | — |

## hicc 处理模式分类

| 模式 | 适用场景 | 占比 |
|------|----------|------|
| ✅ 直接 | 大多数特性（自由函数、类、模板函数、unique_ptr、STL via hicc-std、异常、constexpr）| ~60% |
| 💬 活跃注入 | 类模板 / 模板特化 / 变参模板（在 `hicc::cpp!` 块内 typedef + 包装）| ~10% |
| 💬 注释式注入 | operator 重载、union（在 lib.rs 中以注释形式建议）| ~5% |
| ⚠️ C++ 端调整 | noexcept、variadic `...`、enum class、lambda/std::function | ~20% |
| ⚠️ 限制标注 | 菱形虚继承、std::tuple、std::bind（README 标注，简化处理）| ~5% |
| ❌ 不支持 | 函数指针参数、嵌套模板 `vector<vector<int>>` | — |

## 手动处理方案汇总（按模式归类）

每种处理模式对应一套可机械化的 codegen 模板；下表汇总各模式的输入、产物、关键判定字段。

| 模式 | 触发条件（AST 字段）| 手动操作 | Rust 产物 | C++ 产物 |
|------|----------------------|----------|-----------|----------|
| ✅ 直接 (自由函数) | `FunctionDecl` 无 class context | `import_lib!` `#[cpp(func="ret name(args)")]` | `pub fn name(args) -> ret` | 不动 |
| ✅ 直接 (类) | `CXXRecordDecl` + factory + free pair | `import_class!` + `import_lib!` factory | `pub class T` + factory fn | factory/free pair |
| ✅ 直接 (异常) | `FunctionDecl` body 含 `CXXThrowExpr` | 返回类型从 `T` 改为 `hicc::Exception<T>` | `pub fn f() -> Exception<T>` | 不动 |
| ✅ 直接 (constexpr) | `FunctionDecl.isConstexpr` | 当作普通函数路径 | 同自由函数 | 不动 |
| 💬 活跃注入 (类模板) | `ClassTemplateDecl` | `hicc::cpp!` 内 `using FooInt = Foo<int>;` + factory | `pub class FooInt` | typedef+factory 注入 |
| 💬 活跃注入 (变参模板) | `FunctionTemplateDecl` variadic | `hicc::cpp!` 内固定 arity wrapper | 同自由函数 | wrapper 注入 |
| 💬 注释式注入 (operator) | `CXXMethodDecl` operator+/-/* 等 | C++ 端写命名包装 `t_add(t*,t*)` | 普通自由函数 | 命名 wrapper |
| ⚠️ C++ 端调整 (noexcept) | `FunctionDecl.exceptionSpecType=EST_BasicNoexcept` | **修改 C++ 头**：去掉 `noexcept` | 普通方法绑定 | 头文件签名修改 |
| ⚠️ C++ 端调整 (enum class) | `EnumDecl` (scoped) | C++ 端写 `to_int_*/int_to_*` wrapper | Rust 镜像 `#[repr(i32)] enum` + factory | int 桥接 wrapper |
| ⚠️ C++ 端调整 (lambda/std::function) | `LambdaExpr` / `std::function` 在签名 | C++ 端写命名 wrapper，把 callable 隐藏到 .cpp | 普通自由函数 | 命名 wrapper |
| ⚠️ C++ 端调整 (union) | `CXXRecordDecl.isUnion` | C++ 端 `ValueBox` 包装类 + typed setter/getter + tag int | 普通 class 绑定 | 包装类 |
| ⚠️ 限制标注 (复杂) | 多重虚继承 / `std::tuple` / `std::bind` | README 标注简化处理 | 选择性绑定 | 可能简化 |

## 自动化可行性评估

下表汇总各特性的 rust_gen 自动化难度（参考 `rust_gen/filter.py` 跳过规则与两种产出模式判定）。

| 等级 | 数量 | 代表特性 | 评估依据 |
|------|------|---------|----------|
| **高** | 26 | 001-012, 020-024, 029-030, 033, 036, 042-043, 046 | 单一 AST 字段判定，模板化 codegen 即可 |
| **中** | 16 | 013-017, 025-028, 031-032, 034-035, 037, 044-045, 047 | 多字段联合判定，需要约定 wrapper 命名 |
| **低** | 6 | 005, 018, 019, 038-041, 048 | 需要人工设计包装结构（菱形继承、tuple 拆解、callable 包装）|

**判定规则**（与 `rust_gen/filter.py` 跳过规则对齐）：
- **高**：仅依赖 `FunctionDecl/CXXRecordDecl` 顶级字段，模板化可直接生成；
- **中**：需要二级字段（如 `CXXMethodDecl.isConst`/`exceptionSpecType`），或需要附加 wrapper；
- **低**：需要人工选择降级策略（菱形虚继承、std::tuple 拆 first/second、lambda 命名包装）。

## hicc 限制与降级策略

详见 [`docs/hicc-capabilities.md`](./docs/hicc-capabilities.md) § 限制清单。摘要：
- `noexcept` 成员方法 → C++ 端移除（自由函数可保留）
- 运算符重载 → 命名包装函数
- union → `ValueBox` 包装类
- 变参模板 → 固定 arity 包装
- 类模板 path 含 `<T>` → typedef 后用 `FooInt*`
- enum class → int 转换函数
- lambda / std::function → C++ 端命名包装函数

## 学习路径

1. **入门**：001 → 006 → 013（自由函数 → 类 → 继承）
2. **模板**：024 → 025 → 028（函数模板 → 类模板 → 变参模板）
3. **内存**：029 → 031 → 033（unique_ptr → 自定义删除器 → RAII）
4. **STL**：034 → 036 → 037（vector → string → array）
5. **高级**：042 → 045 → 047（异常 → union → noexcept 限制）

## 许可

本项目仅供学习参考。hicc 子模块的许可参见 `hicc/README.md`。
