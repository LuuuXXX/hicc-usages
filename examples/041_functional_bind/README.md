# 041_functional_bind — functional_bind

## C++ API

演示 `std::bind` + 占位符 (`std::placeholders::_1`) 绑定函数与成员访问。`std::bind` 返回类型不可跨 FFI 命名。

## hicc 支持方式

⚠️ **限制标注：命名包装**

把 `std::bind` 表达式封到具名 wrapper 内（`add_bound_10` / `mul_bound_3` / `sub_bind_first` / `point_x_plus_offset`）。POD-like 结构 `BindPoint` 暴露为 hicc class（factory + free pair）。

## 关键 AST 字段

| 字段 | 提取方式 |
|------|----------|
| `FunctionDecl.name` | wrapper 名 |
| `FunctionDecl.returnType` | 整型，无修饰 |
| `ParmVarDecl` | 含 `const BindPoint*` 形参 → Rust `*const BindPoint` |
| `CXXRecordDecl` (`BindPoint`) | 简单 POD，仅需 factory + free |
| Function body | **丢弃**（`std::bind` 表达式不可跨 FFI） |

## 手工映射步骤

1. 识别 `std::bind` 表达式（AST body 含 `CXXOperatorCallExpr` / `CallExpr` 指向 `std::bind`）。
2. 写具名 wrapper 替换：在 C++ .cpp 内保留 `std::bind`，wrapper 返回最终结果。
3. POD 结构通过 `factory + free` pair 暴露：`bind_point_new` / `bind_point_free`。
4. Rust 端 `import_class! { pub class BindPoint {} }` 声明类型，`import_lib!` 暴露 factory。
5. Pointer-to-class 参数在 Rust 用 `&BindPoint`（hicc 自动转 `*const`）。

## hicc 限制 / 降级

- ❌ `std::bind` 返回类型（`_Bind`）不能直接命名。
- ✅ POD 结构通过 factory/free 可正常导出。
- ✅ Pointer-to-class 跨 FFI 由 hicc 自动处理。

## 自动化评估

**中**：检测 `std::bind` 出现易；自动生成 wrapper 需约定 wrapper 命名模板；POD factory/free 可模板化。

## 构建 / 验证

```bash
cd cpp && bash standalone.sh    # or: make    # or: cmake + make
cd rust_hicc && cargo test
../../scripts/verify-one.sh 041_functional_bind
```
