# 048 summary — AST→hicc 笔记

## AST 关键信息

`summary.h` 在 `summary_ns` 下混合多种 C++ 特性：
- `enum class CustomerTier : int` + int<->enum 转换器
- 类 `Customer`：构造、`std::string` 字段、`std::vector<int>` 成员、`charge`/`purchase_at`/`upgrade` 抛异常
- 单继承体系 `CustomerBase` (有纯虚 `discount()`) → `VipCustomer`（派生，override `label`/`discount`）
- 自由函数 `compute_discounted_price(CustomerBase&, double)` —— 接基类引用
- `doubled_values(const std::vector<int>&) -> std::vector<int>`
- `struct Settings { static constexpr int MAX_CUSTOMERS; static constexpr double DEFAULT_DISCOUNT; }`

## hicc 模式选择（综合 5 种）

| 特性 | 模式 | 来源 |
|------|------|------|
| `Customer` 类 + Exception 方法 | `hicc::Exception<T>` 返回类型 | 042 |
| `CustomerTier` enum class | int 转换器 + Rust 镜像 enum | 044 |
| 单继承 virtual | 仅绑派生类 `VipCustomer` | 015 |
| `CustomerBase&` 参数 | `hicc::cpp!` 块 inline 包装转 `VipCustomer&` | 019 (operator pattern) |
| `Settings` 静态 constexpr | `#[cpp(data = "...")]` → `&'static T` | 046 |
| `std::vector<int>` 参数/返回 | `RustVec = hicc_std::vector<hicc::Pod<i32>>` | 034 |

关键技巧：
- `Customer::tier()` 返回 `CustomerTier`（enum class）—— 用 inline helper `customer_tier_int(const Customer&) -> int` 跨 FFI，Rust 端把 int 转回 `CustomerTier` 镜像
- `compute_discounted_price(CustomerBase&, ...)` —— C++ 侧 `CustomerBase&` 是抽象基类，hicc 不能直接绑；用 inline 包装 `discounted_price_vip(const VipCustomer&, ...)` 转成派生类引用
- `class string = hicc_std::string;` 只能在 `import_class!` 或 `import_lib!` 一处声明（重复定义报 E0428）

## 自动化可行性

- 半自动可行：rust_gen 需要对每种特性应用对应规则：
  - enum class → int 转换器 + 镜像
  - virtual base& 参数 → cpp! inline 包装为派生类引用
  - 抽象基类方法（pure virtual）→ 跳过，仅绑派生类
  - Exception 方法 → 用户标注哪些方法会抛
- 工作量：filter.py 已基本就绪（按 batch A-E 的规则），summary 仅是把多个规则在同一文件里组合应用
