# Implementation Tasks: 48 C++ 特性 → Rust FFI（hicc）矩阵

**Change ID:** `execute-cpp-feature-rust-ffi-matrix`

---

## Phase 0: hicc 能力图谱与基础设施

- [ ] 0.1 通读 `hicc/` 的 9 个子 crate，归纳能力清单到 `docs/hicc-capabilities.md`
  - 列出每个 crate 的入口（`lib.rs` / `build.rs`）、对外暴露的宏、提供的 trait、支持的 C++ 构造
  - 画 9 个 crate 之间的依赖图（hicc-build ↔ hicc-macros ↔ hicc ↔ hicc-std 等）
- [ ] 0.2 在 `docs/hicc-capabilities.md` 中写"C++ 构造支持表"：自由函数 / 类 / 模板 / 继承 / 运算符 / STL / 异常 / RTTI / noexcept / union / lambda 等 → hicc 对应能力
- [ ] 0.3 写"限制清单"：noexcept 成员方法、菱形虚继承、std::tuple、lambda/std::function、嵌套模板等
- [ ] 0.4 实现 `tools/ast-extract/extract.sh`：参数 `<cpp-dir>`，输出 `ast/preprocessed.cpp` + `ast/ast.json`
- [ ] 0.5 实现 `tools/cpp-templates/{standalone.sh.tmpl, Makefile.tmpl, CMakeLists.txt.tmpl}` 三套构建模板
- [ ] 0.6 实现 `tools/scaffold-feature.sh <NNN>_<name>`：生成 4 件套目录骨架
- [ ] 0.7 在 `.gitignore` 中排除 `target/`、`build/`、`*.a`、`*.o`、`ast/preprocessed.cpp`
- [ ] 0.8 实现 `scripts/verify-one.sh <NNN>`：cpp 三套构建 + AST 提取 + cargo build + cargo test

**Quality Gate:**
- [ ] `./tools/scaffold-feature.sh 000_smoke` 生成正确骨架
- [ ] `./tools/ast-extract/extract.sh` 在 hicc-examples/hello-world 上能跑通
- [ ] `docs/hicc-capabilities.md` 覆盖 9 个子 crate

---

## Phase 1: 基础特性（001-005）

- [ ] 1.1 `001_hello_world` — 4 件套 + 端到端 verify 通过
- [ ] 1.2 `002_function_overload`
- [ ] 1.3 `003_default_args`
- [ ] 1.4 `004_inline_functions`
- [ ] 1.5 `005_variadic_functions`（C++ 端固定 arity 包装）

**Quality Gate:**
- [ ] 5/5 `./scripts/verify-one.sh 00X` 通过
- [ ] 5/5 特性 README 含 AST 关键字段 + 手工映射步骤

---

## Phase 2: 类基础（006-012）

- [ ] 2.1 `006_class_basic`（factory + `import_class!` + `destroy=`）
- [ ] 2.2 `007_class_constructor`
- [ ] 2.3 `008_class_copy`（clone 包装）
- [ ] 2.4 `009_class_move`（`T&&` 方法用 self 接收）
- [ ] 2.5 `010_class_static`
- [ ] 2.6 `011_class_const`
- [ ] 2.7 `012_class_volatile`

**Quality Gate:**
- [ ] 7/7 cargo test 通过
- [ ] `import_class!` 的 destroy/factory 模式在所有 7 个特性中一致

---

## Phase 3: 继承与多态（013-018）

- [ ] 3.1 `013_inheritance_single`
- [ ] 3.2 `014_inheritance_multiple`（或 `#[interface]` + `@make_proxy`）
- [ ] 3.3 `015_virtual_basic`
- [ ] 3.4 `016_virtual_pure`
- [ ] 3.5 `017_virtual_override`
- [ ] 3.6 `018_virtual_diamond`（⚠️ 限制标注：简化为组合）

**Quality Gate:**
- [ ] 6/6 cargo test 通过
- [ ] 018 README 明确标注"限制 + 降级方案"

---

## Phase 4: 运算符与特殊成员（019-023）

- [ ] 4.1 `019_operator_overload`（💬 注释式注入：C++ 端写命名包装函数 `vec2_add`）
- [ ] 4.2 `020_friend_function`
- [ ] 4.3 `021_explicit_ctor`（factory 模式）
- [ ] 4.4 `022_mutable_member`
- [ ] 4.5 `023_typeid_rtti`

**Quality Gate:**
- [ ] 5/5 cargo test 通过
- [ ] 019 在 lib.rs 中以注释式注入建议表达"运算符 → 命名包装"

---

## Phase 5: 模板（024-028）

- [ ] 5.1 `024_template_function`（`#[cpp(func="ret f<T>(args)")]`）
- [ ] 5.2 `025_template_class`（💬 活跃注入：`using FooInt = Foo<int>;` + factory）
- [ ] 5.3 `026_template_specialization`
- [ ] 5.4 `027_template_instantiation`
- [ ] 5.5 `028_variadic_template`（💬 活跃注入固定 arity：sum_two/sum_three）

**Quality Gate:**
- [ ] 5/5 cargo test 通过
- [ ] `hicc::cpp! {...}` 块在 025-028 中用法一致

---

## Phase 6: 智能指针与内存（029-033）

- [ ] 6.1 `029_unique_ptr`（返回值类型剥 `unique_ptr<>`）
- [ ] 6.2 `030_shared_ptr`
- [ ] 6.3 `031_custom_deleter`（`destroy="free_func"`）
- [ ] 6.4 `032_placement_new`（factory: `T* construct_at(buf, args)`）
- [ ] 6.5 `033_raii_pattern`（`destroy=` 给 Drop）

**Quality Gate:**
- [ ] 5/5 cargo test 通过
- [ ] 6/6 在 README 中标注"返回类型剥壳"或"自定义 destroy"

---

## Phase 7: STL 容器（034-038）

- [ ] 7.1 `034_vector_basic`（`hicc_std::vector<Pod<T>>`）
- [ ] 7.2 `035_map_basic`
- [ ] 7.3 `036_string_basic`（**必须** `import_class! class string`，**不可**用 `hicc_std::string`）
- [ ] 7.4 `037_array_basic`（typedef `CppArr=std::array<T,N>`）
- [ ] 7.5 `038_tuple_basic`（⚠️ 限制标注：命名 accessor first/second）

**Quality Gate:**
- [ ] 5/5 cargo test 通过
- [ ] 036 README 顶部明示"不要用 hicc_std::string"（参考 hicc Key Patterns feedback）

---

## Phase 8: 函数对象（039-041）

- [ ] 8.1 `039_lambda_basic`（⚠️ C++ 端命名包装函数）
- [ ] 8.2 `040_std_function`（命名包装）
- [ ] 8.3 `041_functional_bind`（⚠️ 限制标注：命名包装）

**Quality Gate:**
- [ ] 3/3 cargo test 通过
- [ ] README 标注"无法直接跨 FFI，必须 C++ 端包装"

---

## Phase 9: 异常与高级（042-048）

- [ ] 9.1 `042_exception_basic`（Rust 返回 `hicc::Exception<T>`）
- [ ] 9.2 `043_namespace_nested`
- [ ] 9.3 `044_enum_class`（int 转换函数）
- [ ] 9.4 `045_union_basic`（💬 注释式注入：ValueBox 包装类）
- [ ] 9.5 `046_constexpr_basic`
- [ ] 9.6 `047_noexcept_basic`（⚠️ **唯一例外**：C++ 端移除成员方法 noexcept）
- [ ] 9.7 `048_summary`（综合多种特性）

**Quality Gate:**
- [ ] 7/7 cargo test 通过
- [ ] 047 README 顶部醒目标注"C++ 端唯一例外"
- [ ] 048 README 链接到所有上述模式

---

## Phase 10: 全量验证 + README

- [ ] 10.1 实现 `scripts/cpp-build-all.sh`：48 特性三套构建全部跑通
- [ ] 10.2 实现 `scripts/rust-test-all.sh`：48 特性 cargo test 全部通过
- [ ] 10.3 重写根 `README.md` 总表：48 特性 × (支持度, 关键 AST 字段, 手工映射, 自动化等级)
- [ ] 10.4 在 README 中加入"手工处理模式总结表"：6 种模式（直接 / 活跃注入 / 注释式注入 / C++端调整 / 限制标注 / 不支持）各自的特征与代表特性
- [ ] 10.5 在 README 中加入"自动化可行性表"：哪些特性的手工步骤可以自动化、哪些不能、为什么
- [ ] 10.6 验证 `hicc/` 子模块 `git status` 无改动（除 047 已记录的 C++ 端调整）

**Quality Gate:**
- [ ] 48/48 `verify-one.sh` 通过
- [ ] `rust-test-all.sh` 显示 48/48 通过
- [ ] README 总表与各特性 README 一致
- [ ] hicc 子模块干净

---

## Completion Checklist

- [ ] 所有 Phase 完成
- [ ] 所有 Quality Gate 通过
- [ ] docs/hicc-capabilities.md 与 README 表格一致
- [ ] hicc 子模块无改动
- [ ] 可以执行 `/openspec-archive execute-cpp-feature-rust-ffi-matrix`
