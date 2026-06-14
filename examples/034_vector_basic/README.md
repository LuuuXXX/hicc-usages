# 034_vector_basic

✅ **直接**：`std::vector<int>` 作为类成员。本例通过 push/size/at 访问器访问，未用 `hicc_std::vector` alias（适合当容器不离开 C++ 边界时）。如需 Rust 直接持有 vector，用 `class Vec = hicc_std::vector<Pod<T>>`。

## 关键 AST 字段
| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 容器类型 | `parameters[].type.qualType` 含 `std::vector<` | 决定用 `hicc_std::vector<Pod<T>>` 或包装 |

## 自动化评估
**中**。包装方法机械，但 `hicc_std::vector` alias 路径需要类型映射规则。

## 构建 / 验证
`../../scripts/verify-one.sh 034`
