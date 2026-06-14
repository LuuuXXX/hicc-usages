# 019_operator_overload: AST → hicc 手动映射

## user-ast.json 关键信息

- `CXXRecordDecl Vec2`
- 普通 method: `x() const`, `y() const`
- operator 重载：`operator+(Vec2) const`, `operator-(Vec2) const`, `operator*(float) const`, `operator+=(Vec2)`, `operator==(Vec2) const`, `operator[](int) const`, `operator-() const`（一元）

## hicc 模式：⚠️ **operator 必须用 cpp! 块包装**

hicc 不支持以 `operator` 开头的方法名直接 import。识别 `CXXMethodDecl.name` 含 `operator` 前缀 → 生成命名空间级 inline 包装函数：

```rust
hicc::cpp! {
    inline Vec2 vec_add(const Vec2& a, const Vec2& b) { return a + b; }
    inline float vec_at(const Vec2& a, int i) { return a[i]; }
}
hicc::import_lib! {
    #[cpp(func = "Vec2 vec_add(const Vec2&, const Vec2&)")]
    pub fn vec_add(a: &Vec2, b: &Vec2) -> Vec2;
}
```

## 自动化可行性：高

`CXXMethodDecl.name` 形如 `operator+/-/* / /==/[]/()` → 机械映射到 wrapper 函数名（`vec_add`/`vec_sub`/`vec_at`/...）。重载的 arity 和返回类型决定 Rust fn 签名。
