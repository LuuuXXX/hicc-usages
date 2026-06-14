# 037_array_basic

✅ **直接（typedef alias）**：`std::array<int, 4>` 通过 typedef `IntArray4` 暴露。

## 关键 AST 字段
| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 模板参数 | `std::array<T,N>` 解析 T, N | 生成 typedef + 绑定 |

## 自动化评估
**中**。

## 构建 / 验证
`../../scripts/verify-one.sh 037`
