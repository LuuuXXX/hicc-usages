# 033_raii_pattern

✅ **直接**：RAII 通过 destroy= 属性接到 Rust Drop。C++ 析构在 destroy 函数内调用 `delete`，析构函数完成清理。

## 关键 AST 字段
| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 析构函数 | `CXXDestructorDecl` | 信息性（确认资源会在析构时释放） |

## 自动化评估
**高**。RAII 是 hicc 的天然支持场景。

## 构建 / 验证
`../../scripts/verify-one.sh 033`
