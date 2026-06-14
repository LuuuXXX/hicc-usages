# 分批规划（避免单批内存爆掉）

每个 Phase 内的 48 项工作切成 6 批，每批 ≤ 8 项。**严格串行，跑完一批清理 target/ 与 build/ 再进下一批**。

## 批次划分（48 项 → 6 批）

| 批次 | 编号范围 | 主题 |
|---|---|---|
| A | 001 - 008 | 函数基础 + 类构造/拷贝 |
| B | 009 - 016 | move/static/const/volatile + 继承基础 |
| C | 017 - 024 | virtual/diamond/operator/friend/explicit/mutable/typeid + 模板函数 |
| D | 025 - 032 | 模板类/偏特化/实例化/变参 + smart ptr 系列 |
| E | 033 - 040 | RAII + STL 容器 + lambda/std::function/bind |
| F | 041 - 048 | functional 收尾 + exception/namespace/enum/union/constexpr/noexcept/summary |

## 每批固定流程

1. **C++ 项目骨架**（Phase 3 子任务）：建 cpp/ 目录、写源码 + 三种构建脚本、跑 `bash standalone.sh` 验证。
2. **AST 导出**（Phase 4 子任务）：跑 `tools/dump_ast.sh`，落 ast/{name}.i + ast/ast.json。
3. **hicc Rust crate**（Phase 5 子任务）：建 rust_hicc/，写 build.rs + lib.rs + tests/smoke.rs + ast-to-hicc-notes.md，跑 `cargo build && cargo test`。
4. **批末清理**：
   ```bash
   find examples/{NNN_start}..{NNN_end} -name target -type d -exec rm -rf {} + 2>/dev/null
   find examples/{NNN_start}..{NNN_end} -name build -type d -exec rm -rf {} + 2>/dev/null
   cargo clean 2>/dev/null
   ```
5. **批末检查**：本批所有项的 standalone.sh exit 0、cargo test exit 0、ast 文件齐全。

## 内存保护原则

- 每批 ≤ 8 项并发 cargo build（实际仍建议 4 项以内并行）。
- AST JSON 通常 1-10 MB，归档到 ast/ 不进 git。
- rust_hicc crate 不进顶层 workspace，避免一次性编译 48 份依赖图。
- 任何批末发现内存 > 80% 占用，立即停下清理 + 减小批大小。
