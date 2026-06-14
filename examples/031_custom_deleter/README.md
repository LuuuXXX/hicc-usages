# 031_custom_deleter

✅ **直接**：`unique_ptr<T, Deleter>` — 把 deleter 暴露为自由函数，作为 destroy= 属性的值。

## 关键 AST 字段
| 字段 | jq 路径 | 用于 |
|------|---------|------|
| Deleter 类型 | `unique_ptr<T, Deleter>` 中的 Deleter | 提取 operator() 等价的自由函数 |

## 自动化评估
**中**。需要识别 Deleter 并映射到等价自由函数。

## 构建 / 验证
`../../scripts/verify-one.sh 031`
