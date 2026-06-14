# 025_template_class: AST → hicc 手动映射

## user-ast.json 关键信息

- `ClassTemplateDecl Stack<T>`（模板声明）
- `CXXMethodDecl Stack<T>::push(const T&)`、`pop()`、`top()`、`size()`
- 显式实例化 `template class Stack<int>;`

## hicc 模式：⚠️ **`const T&` 对原始类型不收**

`push(const T&)` 对 `Stack<int>`：hicc 拒绝 `const int&` 参数。**手动在 cpp! 块里包一个内联包装**，参数改为值传：

```rust
hicc::cpp! {
    inline void stack_int_push_wrap(StackInt& s, int v) { s.push(v); }
}
```

类本身：用 `using StackInt = template_class_ns::Stack<int>;` 起别名后 import_class!。`Stack<std::string>` 同理（StackString 别名），string 的 `const T&` 可用，不需要包装。

## 自动化可行性：中-高

模板类按 `using` 别名生成。**关键判定**：方法签名里 `const T&` 对原始类型 → 必须生成内联包装。对类类型则可直接绑。

```rust
#[cpp(class = "StackInt")]
pub class StackInt {
    #[cpp(method = "int top() const")]
    pub fn top(&self) -> i32;
    // push 通过 cpp! 包装另绑
    pub fn push(&mut self, v: i32) { stack_int_push_wrap(self, v) }
}
```
