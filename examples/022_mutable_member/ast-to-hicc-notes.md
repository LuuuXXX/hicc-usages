# 022_mutable_member: AST → hicc 手动映射

## user-ast.json 关键信息

- `CXXRecordDecl Query`
- `FieldDecl key_` (`std::string`)
- `FieldDecl call_count_` (`int`, **mutable**)
- `FieldDecl last_result_` (`std::string`, **mutable**)
- const method: `execute() const`（内部修改 mutable 字段）
- const method: `call_count() const`, `key() const`

## hicc 模式

C++ `mutable` 在 const 方法内允许修改，FFI 完全透明。const 方法用 `&self` 即可：

```rust
#[cpp(method = "std::string execute() const")]
pub fn execute(&self) -> string;
```

## 自动化可行性：高

`FieldDecl.isMutable` 可读，但对 FFI 不影响。和普通 const 方法处理一致，无需特殊路径。
