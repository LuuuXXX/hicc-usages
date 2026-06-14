# Proposal: 落地 48 个 C++ 特性 → Rust FFI（hicc）手工转换矩阵

**Change ID:** `execute-cpp-feature-rust-ffi-matrix`
**Created:** 2026-06-14
**Status:** Draft

---

## Problem Statement

`hicc-usages` 仓库的 README 已经规划了 48 个 C++ 特性的 Rust FFI 转换矩阵，并通过 `reinit` 提交清空了 `examples/`，但当前 `examples/` **完全为空**（仅目录占位），README 中所有"已完成"的描述都是目标而非现实。

实际差距：

- **0/48** 特性项目存在（README 期望 48/48）
- `docs/hicc-capabilities.md`、`tools/`、`scripts/` 全部缺失
- hicc 子模块虽然完整，但尚未梳理出可执行的能力图谱
- 没有 clang AST 提取流水线
- 没有"参考 cpp2rust-demo 特性 → 落地 hicc FFI"的标准工序
- README 表格是"目标蓝图"，但需要表格里的内容真实落地

谁受影响：

- 维护者：需要一个可复现的、按特性切片的端到端工作流
- 学习者：每个特性应能"看 C++、看 AST、看 Rust 代码、跑测试"
- 后续 `rust_gen` 自动化：需要一份手工黄金参照来评估自动化的覆盖率与难度

## Proposed Solution

把 README 的蓝图按"特性切片"逐个落地为可独立验证的最小项目，每个特性都遵循统一的 4 件套：

1. **C++ 项目**（`cpp/`）— 纯 C++ API（`.h` / `.cpp`）+ demo `main.cpp`，三套构建（standalone / Make / CMake）产出同一 `lib<name>.a`
2. **AST 产物**（`ast/`）— clang `-Xclang -ast-dump=json` 输出 + 宏展开后的预处理 `.cpp`
3. **Rust FFI 项目**（`rust_hicc/`）— 基于 hicc 的 `Cargo.toml` + `build.rs` + `lib.rs` + `tests/smoke.rs`，在 `build.rs` 中链接 C++ 构建出的 `lib<name>.a`
4. **特性 README**（`README.md`）— 记录手动映射步骤、关键 AST 字段、hicc 限制/降级、自动化可行性

整体技术路线（对应需求 1–5）：

| 需求 | 做法 |
|------|------|
| 1. hicc 能力图谱 | 通读 `hicc/{hicc,hicc-build,hicc-std,hicc-rs,hicc-rs-macros,hicc-cbindgen,hicc-macros,hicc-autogen,hicc-examples,hicc-rs-examples}`，归纳到 `docs/hicc-capabilities.md`（"哪个 crate 提供哪个能力 / 9 个 crate 之间的依赖图 / 哪些 C++ 构造如何被支持"） |
| 2. C++ 特性表 | 对照 `../cpp2rust-demo/examples/` 的 48 个特性，按 README 既有的"支持度"五级（✅直接 / 💬活跃注入 / 💬注释式注入 / ⚠️C++端调整 / ⚠️限制标注）重写表格 |
| 3. C++ 项目 + 多构建 | 每个特性目录下放 `cpp/standalone.sh`、`cpp/Makefile`、`cpp/CMakeLists.txt`，三套都产出 `cpp/build/lib<name>.a`，外加 `cpp/run_demo.sh` 验证 |
| 3.2. AST 提取 | `tools/ast-extract/extract.sh` 调用 `clang++ -E -P`（宏展开）+ `clang++ -Xclang -ast-dump=json`，写到每个特性 `ast/` |
| 4. Rust 项目 | 逐特性手工从 AST 提取（`FunctionDecl.name/returnType/parameters`、`CXXRecordDecl.methods` 等），手写 `rust_hicc/{Cargo.toml,build.rs,lib.rs,tests/smoke.rs}`；`build.rs` 链接到 `../../<feature>/cpp/build/lib<name>.a` |
| 4.2. 冒烟测试 | 每个 `rust_hicc/tests/smoke.rs` 用 `cargo test` 跑通"创建 → 调用 → 断言 → drop"核心路径；模板类/特化等参照 hicc-examples 的固定 arity 与 typedef factory 模式 |
| 4.3. 手工步骤 | 每个特性的 `README.md` 固定章节：① AST 关键字段 ② 手工映射步骤 ③ 遇到的限制与降级 ④ 自动化评估 |
| 5. README 表格 | 项目根 `README.md` 用一张总表总结 48 特性 × (支持度, 关键 AST 字段, 手工映射要点, 自动化等级)；hicc 全程只读 |

## Scope

### In Scope

- `docs/hicc-capabilities.md`：9 个子 crate 的能力图谱与依赖关系
- `examples/<NNN>_<feature>/`：48 个独立可构建可测试的特性项目（4 件套）
- `tools/ast-extract/extract.sh`：可复用的 clang AST 提取脚本
- `tools/cpp-templates/{standalone,Makefile,CMakeLists.txt}`：C++ 构建模板
- `tools/scaffold-feature.sh`：新特性脚手架（生成 4 件套目录骨架）
- `scripts/{verify-one,cpp-build-all,rust-test-all}.sh`：端到端验证脚本
- 根 `README.md` 总表 + 48 特性对照表 + 手工步骤总结
- hicc 子模块**只读**：唯一例外是 047，README 已明确标注（noexcept 成员方法需移除 noexcept 才能被 hicc-build 类型匹配）

### Out of Scope

- 自动化生成器 `rust_gen` 的实现（仅在 README 标注"自动化等级：高/中/低"）
- 性能基准（覆盖正确性即可）
- 跨平台 CI（Linux + 本地脚本验证为准）
- 修改 hicc 任何子 crate 的源码
- `cpp2rust-demo` 的迁移（它用裸 `extern "C"`，与 hicc 路径不同）

## Impact Analysis

| 组件 | 是否改动 | 详情 |
|------|----------|------|
| `hicc/` 子模块 | 否 | 全程只读（仅 047 在 C++ 端做一处 noexcept 移除，记入 README） |
| `examples/` | 是（核心） | 新增 48 个特性目录，每个 4 件套 |
| `docs/` | 是 | 新增 `hicc-capabilities.md` |
| `tools/` | 是 | 新增 `ast-extract/`、`cpp-templates/`、`scaffold-feature.sh` |
| `scripts/` | 是 | 新增 `verify-one.sh`、`cpp-build-all.sh`、`rust-test-all.sh` |
| `README.md` | 是 | 重写为可验证的对照表（当前蓝图保留并补全数据） |
| `.gitignore` | 是 | 排除 `target/`、`build/`、`*.a`、`*.o` 等产物 |
| `openspec/` | 是 | 本提案及其后续 archive |

## Architecture Considerations

**特性切片 vs 流水线切片**：选择特性切片（每个特性自包含 4 件套）而非"先全部 C++ → 再全部 AST → 再全部 Rust"，原因：

- 单特性端到端跑通后，能立即给学习者一个完整可对照样例
- AST 字段提取策略与具体特性强相关，集中处理会丢失上下文
- 单特性 cargo test 通过即可作为"已落地"标志，进度可视化

**hicc 只读约束**：把所有"需要 hicc 配合"的逻辑都搬到特性自己的 `rust_hicc/`（`#[cpp(...)]`、`hicc::cpp! {...}` 块、`import_class!`、`destroy="..."`）。047 的 noexcept 是 hicc-build 类型匹配限制，唯一可行解是 C++ 端调整（README 明确记录）。

**直接 vs 注释式注入 vs 活跃注入**（沿用 README 既有的三级分类）：

- ✅ 直接：`#[cpp(func=...)]` / `#[cpp(method=...)]` / `import_class!`
- 💬 活跃注入：在 `hicc::cpp! {...}` 块内 typedef + factory（用于类模板、模板特化、变参模板）
- 💬 注释式注入：lib.rs 中以注释形式标明"此处建议 rust_gen 注入"（用于 operator 重载、union ValueBox）

**模板/STL 的 factory 模式**（参考 hicc-examples/hicc-std 与 reference.md）：

- 类模板 → `using FooInt = Foo<int>;` + `FooInt* create_foo_int(...)`
- STL → `hicc_std::vector<Pod<T>>` alias（**string 例外**，必须 `import_class! class string`，不能用 hicc_std::string）

## Success Criteria

- [ ] `docs/hicc-capabilities.md` 覆盖 9 个子 crate，包含依赖图与 C++ 构造支持表
- [ ] 48/48 特性目录存在，每个含 `cpp/` + `ast/` + `rust_hicc/` + `README.md`
- [ ] 48/48 特性的三套 C++ 构建都产出同一 `lib<name>.a`，`cpp/run_demo.sh` 通过
- [ ] 48/48 特性产出 `ast/ast.json` + `ast/preprocessed.cpp`
- [ ] 48/48 特性 `cargo test` 在 `rust_hicc/` 内通过（冒烟测试覆盖创建/调用/drop）
- [ ] `./scripts/verify-one.sh <NNN>` 端到端通过；`./scripts/rust-test-all.sh` 48/48 通过
- [ ] 根 `README.md` 总表覆盖 48 特性 × (支持度, 关键 AST 字段, 手工映射, 自动化等级)
- [ ] hicc 子模块 `git status` 显示无改动

## Risks & Mitigations

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| hicc-build 类型匹配在某些边界特性（虚继承、std::function）失败 | 中 | 高 | 用 README 既有的"⚠️ 限制标注 / C++ 端调整"降级；记入 docs/hicc-capabilities.md § 限制清单 |
| clang AST 字段命名随版本漂移 | 中 | 中 | 固定 `clang++ ≥ 10`；AST 脚本在 README 中给出关键字段的 jq 路径示例 |
| 模板特化等需要 `hicc::cpp!` 活跃注入的特性，build.rs 写法不易统一 | 中 | 中 | 抽取共享 helper（`tools/hicc-build-helpers.rs` 或 README 模板段），统一三件套写法 |
| 48 特性工作量过大导致后期赶工 | 高 | 高 | 按 README 已分好的 9 个部分切片交付（基础→类→继承→运算符→模板→内存→STL→函数对象→异常），每部分作为独立 Phase |
| 047 是唯一需要 C++ 端调整的特例，容易被误改 | 低 | 中 | 在 047 README 顶部以醒目标语标注，并把"为什么必须改"写清楚 |
| examples 误提交构建产物污染仓库 | 中 | 低 | `.gitignore` 提前排除 `target/`、`build/`、`*.a`、`*.o`、`ast/preprocessed.cpp`（保留 `ast/ast.json`）|
