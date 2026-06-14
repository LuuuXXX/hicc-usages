# 001_hello_world

最简自由函数 + `std::string` 返回值。作为整套流水线的"黄金参照"——证明：

- C++ 三套构建（standalone / Make / CMake）产出同一 `libhello_world.a`
- clang AST 提取流水线工作
- hicc 外部 C++ 库链接模式（`rust_hicc/build.rs` 通过 `cargo::rustc-link-lib` 链到 `../cpp/build/libhello_world.a`）

## C++ API

```cpp
// hello_world.h
std::string hello(const std::string& who);   // "hello world from <who>!"
int add(int a, int b);
```

## 关键 AST 字段

来源：`ast/ast.json`（94MB，含 `<string>` 全展开；用 jq 过滤用户声明）

| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 函数名 | `.name` (在 `kind == "FunctionDecl"` 节点) | Rust 函数命名 |
| 返回类型 | `. returnType.qualType` | Rust 返回类型映射（`std::string` → `string`，`int` → `i32`） |
| 形参类型 | `.parameters[].type.qualType` | Rust 参数类型映射（`const std::string&` → `&string`） |

实例（精简）：
```json
{
  "kind": "FunctionDecl", "name": "hello",
  "returnType": { "qualType": "std::string" },
  "parameters": [{ "type": { "qualType": "const std::string &" } }]
}
```

## 手工映射步骤

1. **C++ 端**：写 `.h` + `.cpp`，三套构建（standalone / Make / CMake）产出 `cpp/build/libhello_world.a`
2. **Rust 端**：
   - `rust_hicc/src/lib.rs` 顶部用 `hicc::cpp! { #include "hello_world.h" }` 把外部头文件带入 adapter（hicc-build 会编译它）
   - `std::string` **必须** 用 `import_class!` 单独绑定（参考 `docs/hicc-capabilities.md` §4.6）：
     - 因为无法给标准类型加静态方法，所以在 `hicc::cpp!` 块里定义两个自由函数 `hicc_string_new` / `hicc_string_free`
     - `destroy = "hicc_string_free"` 属性把它们接到 hicc 的 Drop 路径
   - 自由函数 `hello` / `add` 用 `import_lib!` + `#[cpp(func = "...")]` 绑定，签名要和 C++ 头文件**字符级别一致**
   - 函数和 class 都要写 `pub`，否则集成测试访问不到
3. **build.rs**：用 `hicc_build::Build::include("../cpp")` 让 adapter 找到 `hello_world.h`；`cargo::rustc-link-lib=hello_world` 链接外部 `.a`

## hicc 限制 / 降级

- `std::string` 不能用 `hicc_std::string` alias 绑定（hicc-std 的 string 与 C++ `std::string` 内存布局不兼容，会段错误）
- 构造函数不能用 `#[cpp(ctor = "...")]`（hicc 不支持），必须通过 `import_lib!` 中声明的工厂函数（如 `string_new`）

## 自动化评估

**高**。所有需要的字段都在 AST 顶层 `FunctionDecl` 上，签名能机械映射。需要特判的只有 `std::string` 返回类型——可以做一个固定的 "string 别名" 模板（包含 `import_class!` + factory + destroy）。

## 构建 / 验证

```bash
# C++ 三套构建
cd cpp && bash standalone.sh           # → build/libhello_world.a
# 或: make                              # → build/libhello_world.a
# 或: mkdir -p build && cd build && cmake .. && make

# Rust 冒烟测试
cd ../rust_hicc && cargo test

# 端到端
../../scripts/verify-one.sh 001
```
