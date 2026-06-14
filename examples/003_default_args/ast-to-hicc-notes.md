# 003_default_args: AST → hicc 手动映射记录

## user-ast.json 关键信息

- `greet(const std::string&, int times=1, const std::string& suffix="!")` —— 2 个缺省
- `compute(int, int=10, int=100)` —— 2 个缺省

## hicc 模式选择

hicc 不直接支持「省略尾部参数自动套用 C++ 默认值」。Rust 端需为每种调用 arity 写一个 `#[cpp(func = ...)]`：
- C++ 端**只声明一次完整签名**（带缺省值）
- Rust 端**为每个 arity 写一个函数**，`#[cpp(func = ...)]` 只写到对应位置

## 关键代码段

```rust
#[cpp(func = "int default_args_ns::greet(const std::string&, int, const std::string&)")]
pub fn greet_full(name: &string, times: i32, suffix: &string) -> i32;

#[cpp(func = "int default_args_ns::greet(const std::string&, int)")]
pub fn greet_times(name: &string, times: i32) -> i32;

#[cpp(func = "int default_args_ns::greet(const std::string&)")]
pub fn greet_default(name: &string) -> i32;
```

## AST 提取要点

- clang AST 中 `FunctionDecl` 的 `inner[].inner[].name == "defaultArgOffset"` 或 `defaultArg` 节点表明某参数有缺省值
- 自动化需读出每个参数是否带 `Default` value，然后笛卡尔展开生成 N 个 Rust 函数

## 自动化可行性：中

- 缺省参数的"哪些组合要导出"需要业务判断（这里全展开）
- 模式机械可推：从带默认值的参数索引集合 → 子集枚举 → 每个 arity 一个函数
