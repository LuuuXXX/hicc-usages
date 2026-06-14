# 029_unique_ptr

✅ **直接**：`unique_ptr<T>` 返回类型在 hicc 中被剥壳 — Rust 接收 owned T，Drop 调用 destroy= 指定的 deleter。

## 关键 AST 字段
| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 返回类型 | `returnType.qualType` 含 `std::unique_ptr<...>` | 剥壳为内部类型 |

## 自动化评估
**高**。返回类型剥壳是机械的。

## 构建 / 验证
`../../scripts/verify-one.sh 029`
