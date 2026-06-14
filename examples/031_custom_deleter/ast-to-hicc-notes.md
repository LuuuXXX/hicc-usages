# 031_custom_deleter: AST → hicc 手动映射

## user-ast.json 关键信息

- `ClassTemplateSpecializationDecl unique_ptr<int[], ArrayDeleter>` —— 数组 + 自定义 deleter
- `ClassTemplateSpecializationDecl unique_ptr<FILE, FileCloser>`
- 自由函数 `make_int_array(size_t)`、`read_at(const IntArrayPtr&, size_t)`

## hicc 模式：⚠️ **必须包装成 opaque handle**

`unique_ptr<int[], Deleter>` 与 `unique_ptr<T, Deleter>`（非默认 deleter）**都不能直接跨 FFI**：
- 内部是数组 / FILE*，不是 class
- 自定义 deleter 是类型层面的事，hicc 不知道

**手动方案**：在 C++ 侧定义包装 struct `IntArrayHandle { IntArrayPtr ptr; size_t size; };`，再写 4 个函数：
```cpp
IntArrayHandle* make_int_array_handle(size_t n);
int handle_read_at(const IntArrayHandle* h, size_t i);
size_t handle_size(const IntArrayHandle* h);
void destroy_int_array_handle(IntArrayHandle* h);
```

Rust 端在 cpp! 块用 `void*` 屏蔽 IntArrayHandle*（避免 hicc 把它当 class 注册）：

```rust
hicc::cpp! {
    inline void* cd_make_handle(size_t n) { return custom_deleter_ns::make_int_array_handle(n); }
    inline int cd_handle_read(void* h, size_t i) { return custom_deleter_ns::handle_read_at(reinterpret_cast<const IntArrayHandle*>(h), i); }
    // ...
}
```

Rust 用 `IntArray` RAII 包装 `*mut c_void`，在 Drop 中调 destroy。

## 自动化可行性：中

需识别 unique_ptr 模板参数为「非默认 deleter」或「数组特化」，然后生成 C++ 端 opaque handle struct + 包装函数 + Rust 端 RAII 包装。模式固定，可自动化，但需要 C++ 侧新增代码（即不只修改 Rust 端）。
