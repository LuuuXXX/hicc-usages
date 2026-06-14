# 038_tuple_basic

⚠️ **限制标注**：`std::tuple<Ts...>` 不能直接命名。包装为命名类（Triple）+ first/second/third 访问器。

## 关键 AST 字段
| 字段 | jq 路径 | 用于 |
|------|---------|------|
| tuple 类型 | `qualType` 含 `std::tuple<` | 触发"包装为命名类"流程 |

## 自动化评估
**低**。需要生成访问器方法名约定。

## 构建 / 验证
`../../scripts/verify-one.sh 038`
