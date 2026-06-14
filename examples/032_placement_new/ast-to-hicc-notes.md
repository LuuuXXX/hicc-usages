# 032_placement_new: AST → hicc 手动映射

## user-ast.json 关键信息

- `CXXRecordDecl Buffer`（包装一块原始内存）
- `CXXRecordDecl Payload`（被 placement 构造的对象）
- `CXXNewExpr` `new (buf.raw()) Payload(value)` —— placement new（ placement arg 是 buffer.raw() 的 void*）
- `CXXMemberCallExpr p->~Payload()` —— 显式析构调用
- 自由函数 `Payload* place_payload(Buffer&, int)`、`destroy_payload(Payload*)`

## hicc 模式：⚠️ **void* 包装 + Rust 端 buffer**

placement new 涉及「在已有内存上构造对象」+ 「不释放内存只调析构」。hicc 没法直接跨 FFI 这套语义。**两个绕开手段**：

1. C++ 端新增 `place_payload_raw(void* raw, int value)`（接受 raw 内存指针，不依赖 Buffer&）
2. Rust 端在 cpp! 块用 `void*` 屏蔽 Payload*（避免 hicc 把 Payload* 当类指针注册），手动暴露 value/set/destroy

```rust
hicc::cpp! {
    using BufferH = placement_new_ns::Buffer;
    inline void* cd_place(void* raw, int v) { return placement_new_ns::place_payload_raw(raw, v); }
    inline int cd_value(void* p) { return static_cast<placement_new_ns::Payload*>(p)->value(); }
    inline void cd_set(void* p, int v) { static_cast<placement_new_ns::Payload*>(p)->set(v); }
    inline void cd_destroy(void* p) { placement_new_ns::destroy_payload(static_cast<placement_new_ns::Payload*>(p)); }
}
```

Rust 端 `Placement` RAII 包装：new 时调 cd_place，Drop 时调 cd_destroy。
Buffer 本身用 `using BufferH` 别名 + make_unique 工厂正常绑。

## 自动化可行性：低

需要识别 placement new 模式（CXXNewExpr 带参数）+ 显式析构调用（MemberCallExpr 析构名），并生成全新的 C++ 包装层（不能只用已有 C++ 代码）。涉及识别语义模式而非语法类型，自动化复杂度高，建议作为单独模板生成。
