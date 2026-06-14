# 007_class_constructor

参数化构造：`Point(int x, int y)`。hicc 不能直接绑定构造函数，必须通过 factory `point_new(x, y) -> Point*`。

## C++ API

```cpp
class Point { Point(int, int); int get_x() const; int get_y() const; int manhattan() const; };
Point* point_new(int x, int y);
void   point_free(Point*);
```

## 关键 AST 字段

| 字段 | jq 路径 | 用于 |
|------|---------|------|
| 构造函数 | `CXXConstructorDecl`（在 CXXRecordDecl 内）| 提取参数列表 → factory 签名 |
| 参数列表 | `parameters[].type.qualType` | factory 形参 |

## 手工映射步骤

1. 从 `CXXConstructorDecl` 读取参数类型列表
2. 在 C++ 端写 factory：`Point* point_new(int x, int y) { return new Point(x, y); }`
3. 在 `import_lib!` 中绑定 factory；构造函数本身不出现在 `import_class!` 中

## hicc 限制 / 降级

`#[cpp(ctor = "...")]` 不是 hicc 合法属性 — 所有构造必须通过 factory 自由函数。

## 自动化评估

**高**。构造函数参数列表 → factory 签名是机械的。

## 构建 / 验证

```bash
../../scripts/verify-one.sh 007
```
