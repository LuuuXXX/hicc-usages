# 008_class_copy

拷贝构造：`Box(const Box&)`。hicc 不能直接绑定拷贝构造，需在 C++ 端写 `box_clone(const Box*) -> Box*` 包装函数，内部调用拷贝构造。

## C++ API

```cpp
class Box { explicit Box(int); Box(const Box&); int get() const; void set(int); };
Box* box_new(int v);
Box* box_clone(const Box* src);  // invokes copy ctor
void box_free(Box*);
```

## 关键 AST 字段

| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 拷贝构造 | `CXXConstructorDecl` 且 `isCopyConstructor == true` | 提示需要写 clone 包装 |
| 单参 `const Class&` | 参数 `type.qualType == "const Class &"` | 验证是拷贝构造 |

## 手工映射步骤

1. 从 AST 中识别 `isCopyConstructor == true`
2. C++ 端写 `Box* box_clone(const Box* src) { return new Box(*src); }`
3. 在 `import_lib!` 中绑定 clone 函数：`#[cpp(func = "Box* box_clone(const Box*)")]`

## hicc 限制 / 降级

不能直接绑定拷贝构造 — 必须通过命名包装函数。

## 自动化评估

**高**。检测 `isCopyConstructor` 后机械生成 clone 包装。

## 构建 / 验证

```bash
../../scripts/verify-one.sh 008
```
