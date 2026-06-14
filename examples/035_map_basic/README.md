# 035_map_basic

✅ **直接**：`std::map<std::string, int>` 作为类成员，通过命名方法访问。注意 `std::string` 必须用 `import_class!`（不是 `hicc_std::string`）。

## 关键 AST 字段
同 034，加 `std::map<K,V>` 模板参数识别。

## 自动化评估
**中**。

## 构建 / 验证
`../../scripts/verify-one.sh 035`
