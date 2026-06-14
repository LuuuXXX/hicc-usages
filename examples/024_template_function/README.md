# 024_template_function

✅ **直接**：函数模板的每个显式实例化用 `#[cpp(func = "ret f<T>(args)")]` 绑定。

## 关键 AST 字段
| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 模板形参 | `FunctionTemplateDecl.templateParameters` | 信息性 |
| 显式实例化 | `ClassTemplateSpecializationDecl` / 显式实例化节点 | 列出要绑定的具体类型 |

## 自动化评估
**高**。每个显式实例化机械生成一条绑定。

## 构建 / 验证
`../../scripts/verify-one.sh 024`
