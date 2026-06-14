# 001_hello_world: AST → hicc 手动映射记录

## user-ast.json 提取的关键信息

- `FunctionDecl name=hello_world type="void ()"` (file=hello_world.h, line=6)
- `FunctionDecl name=answer type="int ()"` (file=hello_world.h, line=10)
- `NamespaceDecl name=hello_world_ns`（两者都在该 namespace 下）

## hicc 模式选择

- 两个都是 namespace 内的全局函数 → 用 `import_lib!` + `#[cpp(func = "...")]`
- 不需要 `import_class!`（无类）

## 生成步骤

1. 在 `hicc::cpp!` 块 `#include "hello_world.h"`
2. `import_lib!` 中分别声明 `hello_world()` 与 `answer()`，C++ 签名必须带 namespace `hello_world_ns::`
3. 工厂函数/构造函数：无
4. 类型映射：`void` ↔ Rust 无返回；`int` ↔ `i32`

## 关键代码段

```rust
hicc::cpp! { #include "hello_world.h" }

hicc::import_lib! {
    #![link_name = "hello_world"]

    #[cpp(func = "void hello_world_ns::hello_world()")]
    fn hello_world();

    #[cpp(func = "int hello_world_ns::answer()")]
    fn answer() -> i32;
}
```

## 自动化可行性：高

- AST 中 `FunctionDecl` 的 `name` + `type.qualType` 可直接机械映射
- 命名空间从外层 `NamespaceDecl` 跟踪即可
- 无构造函数/字段/模板，零特例
