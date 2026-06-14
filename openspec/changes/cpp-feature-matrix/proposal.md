# Proposal: C++ 特性矩阵 → hicc 手动生成方案全量重做

**Change ID:** `cpp-feature-matrix`
**Created:** 2026-06-14
**Status:** Draft

---

## Problem Statement

当前 `hicc-usages` 仓库已有的 48 个示例（git 历史可见）已被清空（`git status` 显示全部 `D`）。需要按用户要求**从零重建**，不参考旧方案，覆盖以下目标：

1. **绘制 hicc 能力地图**：搞清 `hicc / hicc-build / hicc-cbindgen / hicc-examples / hicc-macros / hicc-rs / hicc-rs-macros / hicc-std / hicc-autogen` 各子项目提供什么、不能提供什么。
2. **建立 C++ 特性 → hicc 支持方式矩阵**：以 `../cpp2rust-demo/examples` 的 48 个 C++ 特性为基准，每个特性标注作用、hicc 是否支持、通过什么代码模式支持。
3. **每个特性产出独立可构建的 C++ 项目**：支持 standalone（g++ 直接构建）、make、cmake 三种方式，并支持运行验证。
4. **每个特性产出宏展开后的 AST**：用 `clang++ -E -dD` 或 `clang++ -Xclang -ast-dump=json` 在特性目录输出 AST 文件。
5. **手动从 AST 提取关键信息并生成 hicc Rust 项目**：每个特性对应一个 Cargo crate，build.rs 链接前一步构建出的 C++ 静态库，支持 `cargo build` + `cargo test`。
6. **README 给出汇总矩阵**：总结每个特性对应的手动处理方案、提取了哪些 AST 关键信息、是否能自动化。

### 谁受影响
- 用户：需要一份可执行的参考基线，把任意 C++ 项目接进 hicc。
- 未来自动化（rust_gen / autogen）：本方案的「关键 AST 信息 + 手动方案」表是自动化的设计依据。

## Proposed Solution

把整个工作按 **6 个 Phase** 串行推进（不并发，避免内存爆掉）。每个 Phase 在产物落盘后立刻在 README/进度表里勾选，并设独立质量门。

| Phase | 内容 | 是否落盘 |
|---|---|---|
| P1 | hicc 能力地图（`docs/hicc-capabilities.md`） | 是 |
| P2 | 特性矩阵设计（48 项分类、hicc 支持路径初判） | 是 |
| P3 | C++ 特性项目骨架 + 三种构建脚本 + standalone 验证（**分 6 批**，每批 ≤ 8 项） | 是 |
| P4 | 每个特性的 AST 导出（clang `-E -dD` / `-Xclang -ast-dump`） | 是 |
| P5 | 每个特性的 hicc Rust crate（build.rs + lib.rs + tests，**分 6 批**） | 是 |
| P6 | README 汇总矩阵 + 自动化可行性评估表 | 是 |

### 关键设计决策
- **P3/P5 分批**：每批 8 项，跑完一批才进下一批，每批结束清理 `target/`、`build/`。理由：用户明确说"不要一次性并发太大，机器的内存不够"。
- **hicc 只读**：不动 `hicc/` 目录下任何文件。
- **每个特性独立目录**：`examples/{NNN_name}/cpp/` + `examples/{NNN_name}/rust_hicc/`，互不依赖，便于单独构建/验证。
- **rust_hicc 直接链接 cpp 静态库**：build.rs 同时驱动 hicc-build 与 cc::Build（参考 `hicc-examples/hello-world` 和旧 cpp2rust-demo 的 003 模板）。
- **AST 工具固定**：用 `clang++ -E -dD -P` 产出预处理后的源（宏展开），再用 `clang++ -Xclang -ast-dump=json -fsyntax-only` 产出 JSON AST。两者都落到 `examples/{NNN_name}/ast/`。

## Scope

### In Scope
- 48 个 C++ 特性每个都有 cpp/ + rust_hicc/ + ast/ 完整产物。
- hicc 能力地图 + README 矩阵。
- 三种 C++ 构建方式（standalone、make、cmake）+ 验证。
- 每个特性的 AST 导出 + 手动 AST→hicc 方案记录。

### Out of Scope
- 自动化 rust_gen 工具开发（本方案只产出"是否可自动化"评估，不写工具）。
- 修改 hicc 任何子项目。
- 性能基准测试。
- Windows/MSVC 适配（先保证 Linux + gcc/clang + stdc++）。

## Impact Analysis

| Component | Change Required | Details |
|---|---|---|
| `examples/` | Yes | 新建 48 个 `{NNN_name}/` 子目录，每个含 `cpp/`、`rust_hicc/`、`ast/` |
| `docs/hicc-capabilities.md` | Yes | 新建，hicc 能力地图 |
| `README.md` | Yes | 新建汇总矩阵 + 自动化可行性表 |
| `hicc/` 子目录 | **No（只读）** | 严格不动 |
| CI / 工作流 | No | 暂不引入 |
| Cargo workspace | No（暂不引入）| 每个 rust_hicc 是独立 crate，避免 workspace 编译内存爆炸 |

## Architecture Considerations

- **不用 workspace**：48 个 crate 进一个 workspace 一次性编译会爆内存。每个 rust_hicc crate 独立 `Cargo.toml`，构建时单独 `cargo build`。
- **静态库命名约定**：`examples/{NNN_name}/cpp` 构建出 `lib{NNN_name}.a`，rust_hicc 的 build.rs 用 `cargo::rustc-link-lib={NNN_name}` + `cargo::rustc-link-search=../cpp/build`。
- **hicc 路径**：rust_hicc 的 Cargo.toml 用 `hicc = { path = "../../../hicc/hicc", version = "0.2" }`（相对路径），hicc-build 同理。这是 hicc-examples 既有的模式。
- **AST 文件不进 git**：在 `.gitignore` 加 `examples/*/ast/*.json`、`examples/*/ast/*.i`，避免仓库膨胀。

## Success Criteria

- [ ] `docs/hicc-capabilities.md` 覆盖 9 个子项目的核心 API（macro/build/std/cbindgen/rs）。
- [ ] 48 个 `examples/{NNN_name}/` 目录都存在 `cpp/`（含至少 standalone.sh + Makefile + CMakeLists.txt）+ `rust_hicc/`（含 build.rs + Cargo.toml + src/lib.rs + tests/smoke.rs）+ `ast/`。
- [ ] 48 个特性的 `cpp/standalone.sh` + `make -C cpp` + `cmake -B cpp/build cpp && make -C cpp/build` 三种方式至少跑通 1 种（standalone 必须过）。
- [ ] 48 个特性的 `cargo build` + `cargo test` 全部通过。
- [ ] 48 个特性都产出 `ast/preprocessed.i`（宏展开）+ `ast/ast.json`（AST）。
- [ ] README 含两张表：①C++ 特性 × hicc 支持方式 × 关键代码模式；②C++ 特性 × 手动处理方案 × 关键 AST 信息 × 自动化可行性。
- [ ] 整个过程**串行**完成，没有并发跑批导致 OOM。

## Risks & Mitigations

| Risk | Probability | Impact | Mitigation |
|---|---|---|---|
| 单批构建内存爆掉 | Med | High | 每批 ≤ 8 项；每批结束 `cargo clean` + 删 `build/`；AST 文件 .gitignore |
| clang 版本不一致导致 AST 结构差异 | Med | Med | 固定使用一种 clang 调用方式；记录 clang 版本到 docs |
| 部分 C++ 特性 hicc 不支持（如 volatile、RTTI typeid） | High | Med | 在矩阵中明确标"部分支持/不支持"，rust_hicc 侧用 unsafe 包装或跳过部分测试，README 写明限制 |
| 047_noexcept 这类需要 C++ 修改才能跑通 | High | Low | 用户已在记忆中说明 047 是例外，允许 C++ 侧做最小修改 |
| hicc-examples 已编译产物残留干扰 | Low | Low | 不动 hicc/，只读 |
| AST JSON 文件巨大（>10MB）| Med | Low | .gitignore 排除；需要时本地生成 |
