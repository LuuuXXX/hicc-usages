#!/usr/bin/env python3
"""mapping.py - C++ 类型/符号 → Rust 类型/符号 的映射规则"""
import re

# 基本 C++ 类型 → Rust 类型
BUILTIN_MAP = {
    "void": "()",
    "bool": "bool",
    "char": "i8",
    "signed char": "i8",
    "unsigned char": "u8",
    "char8_t": "u8",
    "char16_t": "u16",
    "char32_t": "u32",
    "wchar_t": "i32",
    "short": "i16",
    "short int": "i16",
    "unsigned short": "u16",
    "unsigned short int": "u16",
    "int": "i32",
    "signed int": "i32",
    "unsigned int": "u32",
    "long": "i64",
    "long int": "i64",
    "unsigned long": "u64",
    "unsigned long int": "u64",
    "long long": "i64",
    "long long int": "i64",
    "unsigned long long": "u64",
    "unsigned long long int": "u64",
    "float": "f32",
    "double": "f64",
    "long double": "f64",
    "size_t": "usize",
    "ssize_t": "isize",
    "ptrdiff_t": "isize",
    "intptr_t": "isize",
    "uintptr_t": "usize",
    "int8_t": "i8",
    "uint8_t": "u8",
    "int16_t": "i16",
    "uint16_t": "u16",
    "int32_t": "i32",
    "uint32_t": "u32",
    "int64_t": "i64",
    "uint64_t": "u64",
    "std::size_t": "usize",
    "std::ptrdiff_t": "isize",
}

# 跳过的函数名（C++ 编译器生成或保留）
SKIP_FUNC_NAMES = {"__builtin", "__", "operator", "~"}


def map_type(cpp_type, known_classes=None):
    """把 C++ 类型字符串映射为 Rust 类型字符串

    cpp_type: C++ 类型，如 "const Greeter *", "int", "std::string"
    known_classes: set of class short names（不含 namespace）— 用于决定是否用 hicc 类型包装
    返回: (rust_type, needs_class_wrapper)
    """
    known_classes = known_classes or set()
    t = cpp_type.strip()

    # 基本类型
    if t in BUILTIN_MAP:
        return BUILTIN_MAP[t], False

    # const T&
    m = re.match(r"^const\s+(.+?)\s*&$", t)
    if m:
        inner = m.group(1).strip()
        if inner in BUILTIN_MAP:
            return BUILTIN_MAP[inner], False  # POD 按值传递
        if inner in known_classes:
            return f"&{inner}", True
        return f"&{map_type(inner, known_classes)[0]}", True

    # T&
    m = re.match(r"^(.+?)\s*&$", t)
    if m and not t.startswith("const"):
        inner = m.group(1).strip()
        if inner in known_classes:
            return f"&mut {inner}", True
        return None, False  # 不支持的引用类型

    # const T*
    m = re.match(r"^const\s+(.+?)\s*\*$", t)
    if m:
        inner = m.group(1).strip()
        if inner == "char":
            return "*const i8", False
        if inner == "void":
            return "*const std::ffi::c_void", False
        if inner in BUILTIN_MAP:
            return f"*const {BUILTIN_MAP[inner]}", False
        # class const 指针 hicc-build 无法直接 FFI（需手写包装）— 跳过让 rust_gen 过滤
        return None, False

    # T*
    m = re.match(r"^(.+?)\s*\*$", t)
    if m and not t.startswith("const"):
        inner = m.group(1).strip()
        if inner == "void":
            return "*mut std::ffi::c_void", False
        if inner in BUILTIN_MAP:
            return f"*mut {BUILTIN_MAP[inner]}", False
        # class 指针同上
        return None, False

    # const T* const 复合（少见）
    if t.endswith(" *const"):
        return None, False

    # 字符串相关
    if t in ("std::string", "::std::string"):
        return "string", True  # 需 import_class string
    if t == "const char *":
        return "*const i8", False

    # 已知类类型（按值传递）
    if t in known_classes:
        return t, True

    # 模板实例化（如 std::vector<int>）
    if "<" in t and ">" in t:
        return None, False

    # fallback：未识别
    return None, False


def cpp_class_to_rust(cpp_class_name):
    """类名转 Rust 标识符（折叠命名空间，保留模板实例化别名）"""
    # hicc_usages::foo::Bar → Bar
    name = cpp_class_name.split("::")[-1]
    # 模板实例化 Stack<int> 已经被上层处理为别名 IntStack，这里只去前缀
    return name


def cpp_fn_name_to_rust(name):
    """函数名转 Rust 标识符（去命名空间，转 snake_case）"""
    # hicc_usages::foo::my_func → my_func
    n = name.split("::")[-1]
    return n


def is_factory_method(method):
    """判断方法是否是 factory（static + 返回 T* 或 T 自身）"""
    if not method.get("is_static"):
        return False
    rt = method.get("return_type", "")
    cls = method.get("class", "")
    # 返回 ClassName * 或 ClassName（值返回，hicc 用 make_unique）
    if rt == f"{cls} *" or rt == cls or rt == f"{cls}*":
        return True
    return False


def is_deleter_method(method):
    """判断方法是否是 deleter（static void free(T*)）"""
    if not method.get("is_static"):
        return False
    if method.get("return_type") != "void":
        return False
    if method.get("name") not in ("free", "destroy", "release", "dispose"):
        return False
    params = method.get("params", [])
    cls = method.get("class", "")
    if len(params) != 1:
        return False
    pt = params[0].get("type", "")
    return pt in (f"{cls} *", f"{cls}*", f"{cls} &")


def rust_self_for_method(method):
    """根据 const/volatile/static 决定 Rust self 类型"""
    if method.get("is_static"):
        return None  # associated function, no self
    if method.get("is_const"):
        return "&self"
    return "&mut self"


def cpp_method_signature(method):
    """生成 C++ 方法签名串，用于 #[cpp(method = "...")]

    例如 method "greet" 返回 "void greet() const"
    """
    rt = method.get("return_type", "void")
    if rt == "()":
        rt = "void"
    name = method["name"]
    params = ", ".join(p["type"] for p in method.get("params", []))
    suffix = ""
    if method.get("is_const"):
        suffix += " const"
    if method.get("is_volatile"):
        suffix += " volatile"
    return f"{rt} {name}({params}){suffix}".strip()
