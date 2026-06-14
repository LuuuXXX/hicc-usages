# 040_std_function — std_function

## C++ API

演示 `std::function<int(int,int)>` 作为回调包装器（dispatch via kind tag）以及多步 `std::function` 链式组合。`std::function` 类型本身不可跨 FFI 命名。

## hicc 支持方式

⚠️ **C++ 端调整：命名包装函数**

`std::function` 不暴露到 FFI；逻辑封装到具名自由函数（`run_binary_op` / `compose_then_add_then_mul`）。Rust 仅绑定 wrapper。

## 关键 AST 字段

| 字段 | 提取方式 |
|------|----------|
| `FunctionDecl.name` | wrapper 函数名（`run_binary_op` 等） |
| `FunctionDecl.returnType` | 普通整型，无修饰 |
| `ParmVarDecl` 列表 | 整型参数（包括 `op_kind` 调度参数） |
| Function body | **丢弃**——内部 `std::function` 不可跨 FFI |

## 手工映射步骤

1. AST 中识别出依赖 `std::function` 的函数（参数或返回值含 `std::function`）。
2. 在 C++ 侧重写为具名 wrapper：把 `std::function` 隐藏到 .cpp 内部，签名仅用可命名类型。
3. Rust `import_lib!` 绑定 wrapper；不需要 `import_class!`。
4. 冒烟测试覆盖每个 wrapper 全部分支。

## hicc 限制 / 降级

- ❌ 直接暴露 `std::function<T>` 参数/返回值不可行。
- ✅ 通过 `op_kind`/`int` 调度参数模拟回调选择，FFI 友好。

## 自动化评估

**中**：可以自动检测 `std::function` 出现位置（AST 类型含 `std::function`），自动改写为 dispatch wrapper 需要约定 wrapper 模板（kind 标签枚举），可行但需要分模式 codegen。

## 构建 / 验证

```bash
cd cpp && bash standalone.sh    # or: make    # or: cmake + make
cd rust_hicc && cargo test
# end-to-end
../../scripts/verify-one.sh 040_std_function
```
