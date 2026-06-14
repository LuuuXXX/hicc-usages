#!/usr/bin/env python3
"""filter.py - 符号过滤规则

基于 memory 中记录的 hicc-build 陷阱：
- 跳过私有方法
- 跳过构造/析构/拷贝/移动
- 跳过操作符方法（要求 C++ 端提供命名包装）
- 跳过参数含 T* / const T& 的方法（hicc-build 类型匹配失败）
- 跳过同名重载（保留第一个）
- 跳过 std::* 容器 / 模板 / 编译器内置（例外：std::size_t / std::ptrdiff_t 已映射为 usize/isize）
"""
import re

# 操作符模式
OPERATOR_RE = re.compile(r"^(operator|~)")

SKIP_CLASS_NAMES = {
    "allocator", "char_traits", "iterator", "reverse_iterator",
    "basic_string", "basic_string_view",
    "__",  # 编译器内部
}


def should_skip_function(fn):
    """是否跳过该自由函数"""
    name = fn.get("name", "")
    # 操作符 / 编译器内置
    if OPERATOR_RE.match(name):
        return True, "operator/builtin"
    # 跳过 variadic 函数（C 可变参数）
    if fn.get("is_variadic"):
        return True, "variadic"
    # 检查参数类型
    for p in fn.get("params", []):
        if should_skip_param_type(p.get("type", "")):
            return True, f"unsupported param: {p.get('type')}"
    # 检查返回类型
    rt = fn.get("return_type", "")
    if should_skip_return_type(rt):
        return True, f"unsupported return: {rt}"
    return False, ""


def should_skip_method(method):
    """是否跳过该类方法"""
    name = method.get("name", "")
    if OPERATOR_RE.match(name):
        return True, "operator"
    # 跳过构造/析构（编译器会生成同名 implicit 函数，但显式的也有）
    cls = method.get("class", "")
    if name == cls or name == f"~{cls}":
        return True, "ctor/dtor"
    # 跳过 variadic
    if method.get("is_variadic"):
        return True, "variadic"
    # 参数检查
    for p in method.get("params", []):
        if should_skip_param_type(p.get("type", "")):
            return True, f"unsupported param: {p.get('type')}"
    # 返回类型检查
    rt = method.get("return_type", "")
    if should_skip_return_type(rt):
        return True, f"unsupported return: {rt}"
    return False, ""


def should_skip_param_type(t):
    """参数类型是否跳过（hicc-build 类型匹配限制）"""
    t = t.strip()
    # 跳过模板参数包 Args...
    if "..." in t:
        return True
    # 跳过模板类型（含 < >）
    if "<" in t and ">" in t:
        return True
    # std::size_t / std::ptrdiff_t 等价于内建整数，mapping.py 已映射，允许通过
    if t in ("std::size_t", "::std::size_t", "std::ptrdiff_t", "::std::ptrdiff_t"):
        return False
    # 跳过其他 std:: 类型（容器、string、智能指针等需在 C++ 端包装）
    if "std::" in t or "::std::" in t:
        return True
    # 跳过wchar_t
    if "wchar_t" in t:
        return True
    # 跳过函数指针（含括号）
    if "(" in t and ")" in t and "*" in t:
        return True
    # 跳过 restrict / __restrict
    if "restrict" in t or "__restrict" in t:
        return True
    # 跳过嵌套模板 <<>>
    if "<<" in t or ">>" in t:
        return True
    # 跳过带逗号的（多参数模板）
    if "," in t and "<" in t:
        return True
    # 跳过纯标识符（模板参数 T）：T、T&、T* 等。判断条件：单个大写字母或下划线后跟字母
    base = t.rstrip("&*").strip()
    if base and not base.lower() == base and len(base) == 1 and base.isupper():
        return True  # 单字母大写 = 模板参数 T
    return False


def should_skip_return_type(t):
    """返回类型是否跳过"""
    t = t.strip()
    if not t or t == "void":
        return False
    if "<" in t and ">" in t:
        return True  # 模板返回（如 vector）
    if "std::" in t or "::std::" in t:
        return True
    return False


def should_skip_class(cls):
    """是否跳过该类"""
    name = cls.get("name", "")
    if name in SKIP_CLASS_NAMES:
        return True
    if name.startswith("__"):
        return True
    # union 类跳过（由 rust_gen 的 build_union_suggestions 生成注释式 ValueBox 包装）
    if cls.get("is_union"):
        return True
    # 模板主定义（如 Stack 主模板，未实例化）跳过
    if cls.get("is_template_primary"):
        return True
    # 模板实例化（如 Stack<int>）也跳过 —— hicc-build 不能处理模板语法
    # 用户应提供非模板包装类（如 IntStackWrapper）
    if cls.get("is_template"):
        return True
    return False


def dedup_methods(methods):
    """同名方法只保留第一个（避免 Rust impl 重复）"""
    seen = set()
    out = []
    for m in methods:
        if m["name"] in seen:
            continue
        seen.add(m["name"])
        out.append(m)
    return out


def filter_symbols(symbols, special=None):
    """主过滤入口：根据规则筛选可转换的符号

    symbols: dict from symfilter.py
    special: dict from special.yaml[feature]（可含白名单/黑名单/instantiations）
    返回: filtered dict
    """
    special = special or {}
    known_classes = {c["name"] for c in symbols.get("classes", []) if not should_skip_class(c)}

    kept_funcs = []
    for fn in symbols.get("functions", []):
        skip, reason = should_skip_function(fn)
        if skip:
            continue
        kept_funcs.append(fn)

    kept_classes = []
    for cls in symbols.get("classes", []):
        if should_skip_class(cls):
            continue
        kept_methods = []
        for m in cls.get("methods", []):
            skip, _ = should_skip_method(m)
            if skip:
                continue
            kept_methods.append(m)
        cls["methods"] = dedup_methods(kept_methods)
        cls["known_classes"] = list(known_classes)
        kept_classes.append(cls)

    return {
        "feature": symbols["feature"],
        "namespaces": symbols.get("namespaces", []),
        "functions": kept_funcs,
        "classes": kept_classes,
        "enums": symbols.get("enums", []),
        "known_classes": list(known_classes),
    }
