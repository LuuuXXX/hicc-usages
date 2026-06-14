# 030_shared_ptr

✅ **直接**（同 unique_ptr 剥壳模式）。注意：shared_ptr 的 refcount 在 adapter 内会析构，跨 FFI 后语义变成"唯一所有权"。真正的跨实例共享需要额外的 refcount 维护。

## 关键 AST 字段
| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 返回类型 | `returnType.qualType` 含 `std::shared_ptr<...>` | 剥壳为内部类型 |

## 自动化评估
**高**（剥壳部分）。**低**（refcount 跨 FFI 部分）。

## 构建 / 验证
`../../scripts/verify-one.sh 030`
