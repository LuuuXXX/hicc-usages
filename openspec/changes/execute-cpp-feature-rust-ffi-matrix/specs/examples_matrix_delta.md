# Delta: examples/ 特性矩阵

**Change ID:** `execute-cpp-feature-rust-ffi-matrix`
**Affects:** `examples/`、`docs/`、`tools/`、`scripts/`、`README.md`

---

## ADDED

### Requirement: 每个特性目录必须是自包含的 4 件套

每个 `examples/<NNN>_<feature>/` 包含 4 个固定子项：

- `cpp/` — C++ API（`.h` + `.cpp`）+ demo `main.cpp`，支持 standalone / Make / CMake 三套构建
- `ast/` — clang 宏展开后的 `preprocessed.cpp` + `ast.json`
- `rust_hicc/` — 基于 hicc 的 `Cargo.toml` + `build.rs` + `src/lib.rs` + `tests/smoke.rs`，`build.rs` 链接到 `../cpp/build/lib<name>.a`
- `README.md` — 固定章节：① 特性描述 ② 关键 AST 字段 ③ 手工映射步骤 ④ hicc 限制/降级 ⑤ 自动化评估

#### Scenario: 单特性端到端验证
- GIVEN 某特性目录已按 4 件套规范创建
- WHEN 执行 `./scripts/verify-one.sh <NNN>`
- THEN 三套 C++ 构建产出同一 `lib<name>.a`、AST 文件生成、`cargo test` 在 `rust_hicc/` 通过

#### Scenario: 链接到 C++ 构建产物
- GIVEN C++ 项目已构建出 `cpp/build/lib<name>.a`
- WHEN 执行 `cargo build` 在 `rust_hicc/` 中
- THEN `build.rs` 通过 `cargo::rustc-link-lib` + `cargo::rustc-link-search` 链接到该静态库，且 `cargo test` 通过

---

### Requirement: 三套 C++ 构建产出同一静态库

`cpp/` 下必须同时存在 `standalone.sh`、`Makefile`、`CMakeLists.txt`，三套独立运行都生成 `cpp/build/lib<name>.a`，文件路径一致，便于 Rust 端 `build.rs` 无差别链接。

#### Scenario: standalone 构建
- WHEN 执行 `bash cpp/standalone.sh`
- THEN `cpp/build/lib<name>.a` 存在，且 `cpp/run_demo.sh` 可链接运行

#### Scenario: Make 构建
- WHEN 在 `cpp/` 下执行 `make`
- THEN `cpp/build/lib<name>.a` 存在，与 standalone 产物等价

#### Scenario: CMake 构建
- WHEN 在 `cpp/build/` 下执行 `cmake .. && make`
- THEN `cpp/build/lib<name>.a` 存在，与上述两套等价

---

### Requirement: AST 产物可复现

`tools/ast-extract/extract.sh <cpp-dir>` 调用：

- `clang++ -E -P -std=c++17 <headers>` → `ast/preprocessed.cpp`
- `clang++ -Xclang -ast-dump=json -fsyntax-only <headers>` → `ast/ast.json`

#### Scenario: AST 提取
- GIVEN 一个包含 `.h` / `.cpp` 的 C++ 项目
- WHEN 执行 `./tools/ast-extract/extract.sh <cpp-dir>`
- THEN `ast/preprocessed.cpp` + `ast/ast.json` 同时存在，且 `ast.json` 含 `FunctionDecl` / `CXXRecordDecl` 等节点

---

## ADDED

### Requirement: docs/hicc-capabilities.md 覆盖 9 个子 crate

文档必须包含：

- 9 个子 crate（hicc、hicc-build、hicc-std、hicc-rs、hicc-rs-macros、hicc-cbindgen、hicc-macros、hicc-autogen、hicc-examples + hicc-rs-examples）各自的能力清单
- 它们之间的依赖关系图（workspace 内 `path = "../..."` 引用）
- "C++ 构造 → hicc 能力"对照表（自由函数 / 类 / 模板 / STL / 异常 / RTTI / noexcept / union / lambda）
- "限制清单"（菱形虚继承、嵌套模板、lambda 跨 FFI、noexcept 成员方法 等）

#### Scenario: 维护者查阅能力图谱
- GIVEN 维护者要为新 C++ 构造选择 hicc 路径
- WHEN 查阅 `docs/hicc-capabilities.md`
- THEN 能查到该构造属于哪一类（直接 / 活跃注入 / 注释式注入 / C++端调整 / 限制标注）以及代表特性编号

---

## MODIFIED

### Requirement: 根 README.md 总表必须是验证过的真实数据

README 当前总表是"目标蓝图"。本变更要求每条记录在对应特性 4 件套落地后**回头核对**：

- "hicc 支持度" 列必须与该特性的 `rust_hicc/` 实际写法一致
- "关键 AST 字段" 列必须能在该特性 `ast/ast.json` 中找到对应字段
- "手工映射" 列必须与该特性 README § 手工映射步骤一致
- "自动化" 列必须基于该特性的实际手工复杂度评估

#### Scenario: README 与特性一致
- GIVEN 任一特性编号 N
- WHEN 检查 README 中第 N 行
- THEN 该行的 4 列描述与 `examples/<NNN>_<feature>/` 实际内容一致

---

### Requirement: 新增"手工处理模式总结表"

README 中必须新增一节，按 6 种模式总结：

| 模式 | 适用场景 | 代表特性 | rust_gen 自动化难度 |
|------|----------|----------|---------------------|
| ✅ 直接 | 自由函数 / 类 / 模板函数 | 001 / 006 / 024 | 高 |
| 💬 活跃注入 | 类模板 / 模板特化 / 变参模板 | 025 / 026 / 028 | 中 |
| 💬 注释式注入 | operator 重载 / union | 019 / 045 | 中 |
| ⚠️ C++ 端调整 | noexcept / variadic `...` / enum class / lambda | 005 / 039 / 044 / 047 | 低 |
| ⚠️ 限制标注 | 菱形虚继承 / std::tuple / std::bind | 018 / 038 / 041 | 低 |
| ❌ 不支持 | 函数指针参数 / 嵌套模板 | — | — |

#### Scenario: 维护者评估新特性
- GIVEN 一个新的 C++ 特性待评估
- WHEN 查 README § 手工处理模式总结表
- THEN 能立即识别该特性所属模式，并找到代表特性做参照

---

## REMOVED

(None — README 的蓝图保留并补全数据，不删除任何条目)
