# 032_placement_new

✅ **直接（factory 模式）**：placement new 通过 `T* construct_at(void*, args...)` 暴露。Rust 端管理 buffer。

## 关键 AST 字段
| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 类型大小 | sizeof / sizeof 静态调用 | Rust 端分配 buffer |
| 对齐 | alignof | Rust 端分配 buffer |

## 自动化评估
**低**。需要在 Rust 端手工管理 buffer 和对齐。

## 构建 / 验证
`../../scripts/verify-one.sh 032`
