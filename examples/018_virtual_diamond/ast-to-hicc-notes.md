# 018_virtual_diamond: AST → hicc 手动映射

## user-ast.json 关键信息

- `CXXRecordDecl Device`（虚基类：id/category）
- `CXXRecordDecl InputDevice : virtual Device`（category/read）
- `CXXRecordDecl OutputDevice : virtual Device`（category/write）
- `CXXRecordDecl IOCombo : InputDevice, OutputDevice`（实现所有虚方法 + last_input/last_output）

## hicc 模式：⚠️ **必须用 cpp! 块包装所有方法**

菱形虚继承导致 hicc 的指针到成员转换失败：
```
error: pointer to member conversion via virtual base 'virtual_diamond_ns::Device'
```

解决：所有方法（无论来自虚基类还是非虚基类）都通过 cpp! 块的 inline 自由函数包装，import_lib! 直接绑自由函数。import_class! 仅承载类型 + 工厂。返回 `const std::string&` 改为 by value（避免引用生命周期问题）。

```rust
hicc::cpp! {
    inline std::string diamond_id(const virtual_diamond_ns::IOCombo& c) { return c.id(); }
    inline int diamond_read(virtual_diamond_ns::IOCombo& c) { return c.read(); }
}
hicc::import_class! {
    #[cpp(class = "virtual_diamond_ns::IOCombo")]
    pub class IOCombo {
        pub fn new(id: &string) -> Self { iocombo_new(id) }
        pub fn id(&self) -> string { diamond_id(self) }
        pub fn read(&mut self) -> i32 { diamond_read(self) }
    }
}
```

## 自动化可行性：中

判定 `CXXRecordDecl.bases[*].isVirtual == true` 即菱形虚继承 → 强制走 cpp! 包装路径。所有方法（含继承自基类的）都要包装，**不能漏**。
