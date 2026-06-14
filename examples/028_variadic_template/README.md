# 028_variadic_template

💬 **活跃注入固定 arity**：变参模板（`template <typename... Ts>`）不能直接跨 FFI。C++ 端写固定 arity 包装（`sum2`、`sum3`、`sum4`），Rust 绑定包装。

## 关键 AST 字段
| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 参数包 | `TemplateTypeParmDecl` 中 `isParameterPack` | 信息性（提示需要固定 arity 包装） |

## 自动化评估
**中**。需要人工决定生成几个包装、哪些 arity。

## 构建 / 验证
`../../scripts/verify-one.sh 028`
