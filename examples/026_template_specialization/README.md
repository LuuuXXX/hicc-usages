# 026_template_specialization

💬 **活跃注入**：模板特化（部分/全）不能直接命名。C++ 端写命名包装函数调用特化的静态方法。

## 关键 AST 字段
| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 特化节点 | `ClassTemplateSpecializationDecl` | 信息性 |

## 自动化评估
**中**。需要识别每个特化并生成包装。

## 构建 / 验证
`../../scripts/verify-one.sh 026`
