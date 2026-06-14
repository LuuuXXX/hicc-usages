# 036_string_basic

✅ **直接** — **但⚠️ KEY PATTERN**：`std::string` **必须**用 `import_class!` 绑定，**不可**用 `hicc_std::string` alias（内存布局不兼容，会段错误）。

## 关键 AST 字段
| 字段 | jq 路径 | 用于 |
|------|---------|------|
| string 返回/参数 | `qualType` 含 `std::string` | 触发 std::string class 绑定 |
| c_str 方法 | `c_str() const` | 返回 `const char*` 给 Rust |

## 自动化评估
**高**（机械）。但需要确保工具不误用 `hicc_std::string`。

## 构建 / 验证
`../../scripts/verify-one.sh 036`
