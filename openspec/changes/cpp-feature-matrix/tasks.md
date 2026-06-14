# Implementation Tasks: C++ 特性矩阵 → hicc 手动生成方案

**Change ID:** `cpp-feature-matrix`

---

## Phase 1: hicc 能力地图（无构建，纯文档）

- [x] 1.1 通读 `hicc/reference.md`，提取所有宏 + builtin 的用法清单
- [x] 1.2 阅读 `hicc/hicc/src/lib.rs`、`hicc/hicc-macros/src/lib.rs`，列出 `cpp!` / `import_lib!` / `import_class!` 接受的所有属性 (`#[cpp(...)]`、`#[method(...)]`、`#[interface(...)]`) 与内置函数 (`@make_proxy` / `@dynamic_cast` / `@placement_new`)
- [x] 1.3 阅读 `hicc/hicc-build/src/lib.rs`，列出 `Build::new()` 链式 API + `cc::Build` 集成方式
- [x] 1.4 阅读 `hicc/hicc-std/src/*.rs`，列出已封装的 STL 容器：`string / vector / array / map / unordered_map / set / unordered_set / deque / list / forward_list / queue / stack`
- [x] 1.5 阅读 `hicc/hicc-cbindgen`、`hicc/hicc-rs`、`hicc/hicc-rs-macros`、`hicc/hicc-autogen` 的 README，归纳它们各自面向的场景
- [x] 1.6 汇总到 `docs/hicc-capabilities.md`：每个能力 → 触发方式 → 示例代码片段 → 适用场景

**Quality Gate:**
- [x] `docs/hicc-capabilities.md` 涵盖 9 个子项目
- [x] 每个宏/builtin 至少 1 段可读代码示例

---

## Phase 2: 特性矩阵设计（设计稿，无代码）

- [x] 2.1 列出 48 个 C++ 特性（参考 `../cpp2rust-demo/examples/` 编号 001-048）
- [x] 2.2 每个特性初判 hicc 支持方式（直接 import_lib / import_class / interface / hicc-std / 不支持需绕过）
- [x] 2.3 输出 `docs/feature-matrix-draft.md`：48 行 × (特性名 / C++ 作用 / hicc 支持路径 / 关键代码片段)
- [x] 2.4 按 hicc 支持路径分 6 批（每批 ≤ 8 项），写入 `openspec/changes/cpp-feature-matrix/examples-grouping/batches.md`

**Quality Gate:**
- [x] 48 行全覆盖
- [x] 每批 ≤ 8 项，没有跨批依赖

---

## Phase 3: C++ 特性项目骨架（**串行 6 批**）

> ⚠️ 每批结束必须执行 `cargo clean` + `rm -rf examples/*/cpp/build examples/*/rust_hicc/target`，再进下一批。

### 批次 A：001-008（函数与类基础）
- [x] 3.A.1 ~ 3.A.8 为 `001_hello_world` ~ `008_class_copy` 每个创建 `examples/{NNN_name}/cpp/` 目录
- [x] 3.A.9 每个 cpp/ 包含：`{name}.h`（extern "C" 接口）、`{name}.cpp`（实现）、`main.cpp`（独立可执行）、`standalone.sh`（g++ 直接编译 + 跑）、`Makefile`、`CMakeLists.txt`
- [x] 3.A.10 跑通 `bash standalone.sh` 验证 8 项全部可运行

### 批次 B：009-016（move / static / const / volatile / 继承 / 多继承）
- [x] 3.B.1 ~ 3.B.8 同上结构
- [x] 3.B.9 跑通 standalone 验证

### 批次 C：017-024（virtual / diamond / operator / friend / explicit / mutable / typeid / 模板函数）
- [x] 3.C.1 ~ 3.C.8 同上
- [x] 3.C.9 跑通 standalone

### 批次 D：025-032（模板类 / 偏特化 / 实例化 / 变参模板 / unique_ptr / shared_ptr / 自定义 deleter / placement new）
- [x] 3.D.1 ~ 3.D.8 同上
- [x] 3.D.2 跑通 standalone

### 批次 E：033-040（RAII / STL 容器 / lambda / std::function / bind）
- [x] 3.E.1 ~ 3.E.8 同上
- [x] 3.E.9 跑通 standalone

### 批次 F：041-048（functional 收尾 / 异常 / namespace / enum / union / constexpr / noexcept / summary）
- [x] 3.F.1 ~ 3.F.8 同上（047 允许最小修改 C++ 源以兼容 hicc）
- [x] 3.F.9 跑通 standalone

**Phase 3 Quality Gate:**
- [x] 48 个 cpp/ 目录都存在
- [x] 48 个 standalone.sh 全部 exit 0
- [x] 抽样至少 6 个跑 `make` + `cmake` 全通过

---

## Phase 4: AST 导出（**串行 6 批**，跟着 P3 顺序）

- [x] 4.1 写脚本 `tools/dump_ast.sh`：输入 cpp 目录，输出 `ast/{name}.i`（`clang++ -E -dD -P`）+ `ast/ast.json`（`clang++ -Xclang -ast-dump=json -fsyntax-only`）
- [x] 4.2 对每个特性的 cpp 跑一次该脚本，落到 `examples/{NNN_name}/ast/`
- [x] 4.3 在 `.gitignore` 加 `examples/*/ast/*.json` 与 `examples/*/ast/*.i`
- [x] 4.4 抽样核对 6 项 AST 中关键字段（FunctionDecl / CXXRecordDecl / CXXMethodDecl / TemplateSpecialization）齐全

**Phase 4 Quality Gate:**
- [x] 48 个 ast/ 目录都有 2 个产物文件
- [x] 抽样 6 项 AST 字段完整

---

## Phase 5: hicc Rust crate（**串行 6 批**，跟着 P3 分组）

> 每批 ≤ 8 项；每批跑完 `cargo test` 后 `cargo clean` 再进下一批。

### 批次 A：001-008 → rust_hicc
- [x] 5.A.1 每个 `examples/{NNN_name}/rust_hicc/` 包含 `Cargo.toml`（依赖 `hicc`、`hicc-build` 路径指向 `../../../hicc/{hicc,hicc-build}`）、`build.rs`（驱动 `cc::Build` 编译 `../cpp/{name}.cpp` + `hicc_build::Build::new().rust_file("src/lib.rs").compile("{name}")`）、`src/lib.rs`（`hicc::cpp!` + `hicc::import_lib!` / `import_class!`）、`tests/smoke.rs`
- [x] 5.A.2 在每个特性目录写 `ast-to-hicc-notes.md`：列出从 AST 手动提取了哪些信息（函数签名 / 类成员 / 模板参数等）、用什么 hicc 模式承载、关键代码段
- [x] 5.A.3 跑 `cargo build && cargo test`，全部通过

### 批次 B-F：同结构，按 P3 分组顺序推进
- [x] 5.B.1 ~ 5.B.3（009-016）
- [x] 5.C.1 ~ 5.C.3（017-024）
- [x] 5.D.1 ~ 5.D.3（025-032）
- [x] 5.E.1 ~ 5.E.3（033-040）
- [x] 5.F.1 ~ 5.F.3（041-048）

**Phase 5 Quality Gate:**
- [x] 48 个 rust_hicc crate 全部 `cargo build && cargo test` 通过
- [x] 48 个 `ast-to-hicc-notes.md` 完整记录手动方案

---

## Phase 6: README 矩阵 + 自动化评估

- [x] 6.1 在仓库根 `README.md` 写两张表：
  - 表 1：`特性 / C++ 作用 / hicc 支持（Y/部分/N）/ hicc 模式（import_lib/import_class/interface/hicc-std/Pod/绕过）/ 关键代码片段`
  - 表 2：`特性 / 手动处理方案要点 / 从 AST 提取的关键信息（FunctionDecl 字段/CXXMethodDecl 字段/...）/ 自动化可行性（高/中/低 + 理由）`
- [x] 6.2 写明整体进度勾选状态与已知限制（如 047 的特殊性、volatile 的局限性）
- [x] 6.3 在 README 顶部加"如何运行单个特性"和"如何重跑某一批"两个速查命令块

**Phase 6 Quality Gate:**
- [x] README 两张表各 48 行
- [x] 每行的"自动化可行性"给出明确理由（哪段 AST → 哪段 hicc 输出是确定性的）

---

## Completion Checklist

- [x] 全部 6 个 Phase 完成
- [x] 所有质量门通过
- [x] README 矩阵与 docs/ 文档同步
- [x] 整个过程没有任何一批并发跑超过 8 项
- [x] `hicc/` 目录未做任何修改（用 `git status hicc/` 验证）
- [x] 准备 `/openspec-archive`
