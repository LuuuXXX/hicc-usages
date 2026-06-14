# 016_virtual_pure: AST → hicc 手动映射

## user-ast.json 关键信息

- `CXXRecordDecl Storage`（纯虚 put/get/remove/size，含默认 dump）
- `CXXRecordDecl InMemoryStorage : Storage`（实现所有纯虚）

## hicc 模式

抽象基类无法实例化，只绑定具体派生类 InMemoryStorage。纯虚方法在派生类已实现，直接 `#[cpp(method = "...")]`。

```rust
#[cpp(class = "virtual_pure_ns::InMemoryStorage")]
pub class InMemoryStorage {
    #[cpp(method = "bool put(const std::string&, const std::string&)")]
    pub fn put(&mut self, key: &string, value: &string) -> bool;
    #[cpp(method = "size_t size() const")]
    pub fn size(&self) -> usize;
    // ...
}
```

## 自动化可行性：高

判定 `CXXRecordDecl.isAbstract() == true` → 跳过该类，扫描其具体派生类。`size_t` → `usize`，`bool` → `bool`，`std::string` 返回值 → `string`（按值返回需走 `string` 而非 `&string`）。
