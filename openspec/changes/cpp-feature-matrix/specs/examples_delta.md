# Delta: examples/ + docs/

**Change ID:** `cpp-feature-matrix`
**Affects:** `examples/`, `docs/`, `README.md`, `.gitignore`, `tools/`

---

## ADDED

### Requirement: 48 个独立 C++ 特性示例目录

每个 C++ 特性对应 `examples/{NNN_name}/` 一个目录，包含完整的 C++ 项目 + Rust FFI 项目 + AST 产物。

#### Scenario: 单个特性的目录结构
- GIVEN 48 个 C++ 特性列表（001-048）
- WHEN 开发者为某特性建目录
- THEN 目录必须包含：
  - `cpp/{name}.h`（带 `extern "C"` 接口）
  - `cpp/{name}.cpp`（实现）
  - `cpp/main.cpp`（独立可执行入口）
  - `cpp/standalone.sh`（g++ 直接编译并运行）
  - `cpp/Makefile`
  - `cpp/CMakeLists.txt`
  - `rust_hicc/Cargo.toml`（依赖 `hicc`、`hicc-build`）
  - `rust_hicc/build.rs`（驱动 cc::Build + hicc_build::Build）
  - `rust_hicc/src/lib.rs`（`hicc::cpp!` + `import_lib!` / `import_class!`）
  - `rust_hicc/tests/smoke.rs`
  - `ast/{name}.i`（宏展开后源码，.gitignore）
  - `ast/ast.json`（clang JSON AST，.gitignore）
  - `ast-to-hicc-notes.md`（手动 AST→hicc 方案记录）

#### Scenario: C++ 三种构建方式
- GIVEN 任一特性的 cpp/ 目录
- WHEN 执行 `bash standalone.sh` / `make -C cpp` / `cmake -B cpp/build cpp && make -C cpp/build`
- THEN 至少 standalone.sh 必须退出码 0；make / cmake 抽样验证至少 6 项通过

#### Scenario: Rust 构建与测试
- GIVEN 任一特性的 rust_hicc/ 目录
- WHEN 执行 `cargo build && cargo test`
- THEN 退出码 0，smoke.rs 中至少 1 个测试通过

---

### Requirement: hicc 能力地图文档

新增 `docs/hicc-capabilities.md`，覆盖 9 个子项目的核心 API。

#### Scenario: 文档结构
- GIVEN hicc 仓库的 9 个子项目（hicc / hicc-build / hicc-cbindgen / hicc-examples / hicc-macros / hicc-rs / hicc-rs-macros / hicc-std / hicc-autogen）
- WHEN 阅读 `docs/hicc-capabilities.md`
- THEN 必须能找到：
  - 每个子项目的目标与边界
  - 每个宏（`cpp!` / `import_lib!` / `import_class!`）的属性全集
  - 每个 builtin（`@make_proxy` / `@dynamic_cast` / `placement_new`）的触发方式
  - hicc-std 已封装的 STL 容器清单
  - hicc-build 的 Build API 与 cc::Build 集成方式
  - 至少 1 段可读代码示例 per 能力

---

### Requirement: README 特性矩阵

仓库根 `README.md` 包含两张汇总表。

#### Scenario: 表 1 - C++ 特性 × hicc 支持方式
- GIVEN 48 个 C++ 特性
- WHEN 阅读 README
- THEN 必须看到一张 48 行的表，列：特性名 / C++ 作用 / hicc 支持（Y/部分/N）/ hicc 模式 / 关键代码片段

#### Scenario: 表 2 - 手动方案 × 自动化可行性
- GIVEN 48 个特性的手动 AST→hicc 处理记录
- WHEN 阅读 README
- THEN 必须看到一张 48 行的表，列：特性 / 手动处理方案要点 / 关键 AST 信息 / 自动化可行性（高/中/低 + 理由）

---

### Requirement: AST 导出工具

新增 `tools/dump_ast.sh`：输入 cpp 目录，输出宏展开后的源与 JSON AST。

#### Scenario: 工具调用
- GIVEN 一个 `examples/{NNN_name}/cpp/` 目录
- WHEN 执行 `bash tools/dump_ast.sh examples/{NNN_name}/cpp/{name}.cpp`
- THEN 在 `examples/{NNN_name}/ast/` 下生成 `{name}.i` 与 `ast.json`
- AND 两个文件被 `.gitignore` 排除

---

## MODIFIED

### Requirement: .gitignore

在仓库根 `.gitignore` 追加以下条目，避免构建产物与 AST 文件污染仓库：

```
examples/*/cpp/build/
examples/*/rust_hicc/target/
examples/*/ast/*.i
examples/*/ast/*.json
```

#### Scenario: 构建产物不进 git
- GIVEN 任意特性已构建过
- WHEN `git status`
- THEN ast/*.i、ast/*.json、cpp/build/、rust_hicc/target/ 不出现在变更列表

---

## REMOVED

（无 —— 本方案是从零重建，无既有需求被删除）
