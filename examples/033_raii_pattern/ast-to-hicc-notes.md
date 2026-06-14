# 033_raii_pattern: AST → hicc 手动映射

## user-ast.json 关键信息

- `CXXRecordDecl FileHandle`：成员 `fd_`、`path_`、`written_`，方法 `fd()` / `path()` / `write()` / `size()`
- 构造函数 `FileHandle(int, const std::string&)`
- 工厂 `std::unique_ptr<FileHandle> open_file(int, const std::string&)`
- 辅助 `long read_file(FileHandle&)` —— 接受引用而非 unique_ptr，规避 hicc 已知 bug

## hicc 模式：✅ `import_class!` + `make_unique` 工厂

复用 029 的模式：`make_unique<T, Args...>(Args&&...)` 返回类对象本身（不是 unique_ptr 包装）。
Rust 端 `FileHandle` 的 `Drop` trait（自动生成）= C++ 端 unique_ptr 的释放 → 触发 FileHandle 析构
= RAII 资源回收。不需要单独的 consume 绑定。

```rust
#[cpp(func = "std::unique_ptr<FileHandle> hicc::make_unique<FileHandle, int, const std::string&>(int&&, const std::string&)")]
pub fn open_file(fd: i32, path: &hicc_std::string) -> FileHandle;
```

## 自动化可行性：高

只要识别"类带构造函数 + 返回 `unique_ptr<T>` 的工厂"即可生成 RAII 模板。
构造函数需要 hicc 推断出 `Args...` 对应的 `make_unique<T, Args...>` 实例化。
