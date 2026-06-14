# 027_template_instantiation

💬 **活跃注入**：显式实例化（`template class Stack<int>;`）— 同 025 模式，typedef + factory。

## 关键 AST 字段
| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 显式实例化 | `ClassTemplateSpecializationDecl` 且 `isExplicitSpecialization` | 列出要绑定的特化 |

## 自动化评估
**中**。

## 构建 / 验证
`../../scripts/verify-one.sh 027`
