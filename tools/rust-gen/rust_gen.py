#!/usr/bin/env python3
"""rust_gen.py - 从 symbols.json 生成 Rust crate

用法:
    rust_gen.py --symbols examples/NNN_xxx/ast/symbols.json \
                --out examples/NNN_xxx/rust \
                --special tools/rust-gen/special.yaml

输出:
    <out>/Cargo.toml
    <out>/build.rs
    <out>/src/lib.rs
    <out>/tests/smoke.rs
"""
import argparse
import json
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from filter import filter_symbols
from mapping import (
    map_type, cpp_class_to_rust, cpp_fn_name_to_rust,
    is_factory_method, is_deleter_method, rust_self_for_method,
)
from render import render_template
from special_handlers import (
    build_operator_suggestions, build_union_suggestions,
    build_template_ffis,
    build_class_template_instances,
    build_template_static_wrappers, build_variadic_template_wrappers,
)


def load_special(yaml_path, feature):
    """加载 special.yaml 中该 feature 的策略（含 _default 合并）"""
    import yaml
    data = yaml.safe_load(Path(yaml_path).read_text()) or {}
    default = data.get("_default", {})
    feat = data.get(feature, {})
    merged = {**default, **feat}
    return merged


def _qualify_class_types(type_str, cls_full, known_classes_full):
    """把类型字符串中的 class 短名替换为完整限定名

    type_str: 如 "Greeter *" 或 "const Foo &"
    cls_full: 当前类的完整限定名（用于替换同名短名）
    known_classes_full: 所有已知类的完整限定名集合
    """
    cls_short = cls_full.split("::")[-1] if cls_full else ""
    if cls_short and cls_short in type_str:
        type_str = re.sub(rf"(?<![:\w]){re.escape(cls_short)}(?![\w:])", cls_full, type_str)
    for full in known_classes_full:
        short = full.split("::")[-1]
        if short and short in type_str and short != cls_short:
            type_str = re.sub(rf"(?<![:\w]){re.escape(short)}(?![\w:])", full, type_str)
    return type_str


def cpp_full_method_sig(cls_full, method, known_classes_full=None):
    """生成完整 C++ 方法签名（用于 #[cpp(method = "...")]）

    hicc-build 会把签名中的类型作为 C++ 表达式插入 wrapper，因此所有 class 类型必须用完整限定名。
    """
    known_classes_full = known_classes_full or []
    rt = method.get("return_type", "void")
    if rt == "()":
        rt = "void"
    rt = _qualify_class_types(rt, cls_full, known_classes_full)
    name = method["name"]
    params = ", ".join(
        _qualify_class_types(p["type"], cls_full, known_classes_full)
        for p in method.get("params", [])
    )
    suffix = ""
    if method.get("is_const"):
        suffix += " const"
    if method.get("is_volatile"):
        suffix += " volatile"
    return f"{rt} {name}({params}){suffix}".strip()


def cpp_full_func_sig(fn, known_classes_full=None):
    """自由函数完整 C++ 签名（class 类型用完整限定名）"""
    known_classes_full = known_classes_full or []
    rt = fn.get("return_type", "void")
    if rt == "()":
        rt = "void"
    rt = _qualify_class_types(rt, "", known_classes_full)
    name = fn["full_name"]
    params = ", ".join(
        _qualify_class_types(p["type"], "", known_classes_full)
        for p in fn.get("params", [])
    )
    return f"{rt} {name}({params})"


def cpp_full_factory_sig(cls_full, method, known_classes_full=None):
    """factory static 方法的完整 C++ 签名（返回类型用完整限定名）"""
    known_classes_full = known_classes_full or []
    rt = method.get("return_type", "")
    rt = _qualify_class_types(rt, cls_full, known_classes_full)
    name = f"{cls_full}::{method['name']}"
    params = ", ".join(
        _qualify_class_types(p["type"], cls_full, known_classes_full)
        for p in method.get("params", [])
    )
    return f"{rt} {name}({params})"


def map_param(p, known_classes):
    """映射单个参数：返回 (rust_type, ok)"""
    rt, _ = map_type(p.get("type", ""), known_classes)
    return rt


def unique_param_names(params):
    """参数名去重：同名时改用 arg{i}（处理 Args... 展开为 args,args 的情况）"""
    seen = set()
    out = []
    for i, p in enumerate(params):
        name = p.get("name") or f"arg{i}"
        if name in seen:
            name = f"arg{i}"
        seen.add(name)
        out.append({"name": name, "type": p.get("type", "")})
    return out


def map_return(t, known_classes, cls_name=None):
    """映射返回类型：返回 (rust_type, ok)"""
    if t == "void" or t == "" or t == "()":
        return "()", True
    rt, _ = map_type(t, known_classes)
    if rt is None:
        return None, False
    # hicc 约定：返回 class 类型按值（hicc 内部用 Box 装箱）
    return rt, True


def build_method_context(cls, known_classes, known_classes_full):
    """生成单个 class 的方法/factory 上下文"""
    cls_full = cls["full_name"]
    rust_name = cpp_class_to_rust(cls_full)
    factory_methods = []
    destroy_method = None
    normal_methods = []

    for m in cls.get("methods", []):
        if is_deleter_method(m):
            destroy_method = m
            continue
        if is_factory_method(m):
            factory_methods.append(m)
            continue
        if m.get("is_static"):
            factory_methods.append(m)
            continue
        m_ctx = build_one_method(m, known_classes, rust_name, cls_full, known_classes_full)
        if m_ctx is None:
            continue
        normal_methods.append(m_ctx)

    factory_funcs = []
    for fm in factory_methods:
        ff = build_factory_func(fm, cls_full, rust_name, known_classes, known_classes_full)
        if ff:
            factory_funcs.append(ff)

    destroy = None
    if destroy_method:
        destroy = f"{cls_full}::{destroy_method['name']}"

    return {
        "rust_name": rust_name,
        "cpp_full_name": cls_full,
        "destroy": destroy,
        "methods": normal_methods,
        "factory_funcs": factory_funcs,
    }


def build_one_method(m, known_classes, cls_rust_name, cls_full, known_classes_full):
    """生成普通方法（&self 或 &mut self）的上下文"""
    rust_self = rust_self_for_method(m)
    if rust_self is None:
        return None

    rust_params = []
    rust_param_decl_parts = []
    params = unique_param_names(m.get("params", []))
    for i, p in enumerate(params):
        rt = map_param(p, known_classes)
        if rt is None:
            return None
        pname = p.get("name") or f"arg{i}"
        rust_params.append(rt)
        rust_param_decl_parts.append(f"{pname}: {rt}")

    rt_ret, ok = map_return(m.get("return_type", "void"), known_classes, cls_rust_name)
    if not ok:
        return None
    rust_return = rt_ret

    return {
        "rust_name": cpp_fn_name_to_rust(m["name"]),
        "rust_self": rust_self,
        "rust_param_decl": ", ".join(rust_param_decl_parts),
        "rust_return": rust_return,
        "cpp_method_sig": cpp_full_method_sig(cls_full, m, known_classes_full),
    }


def build_factory_func(fm, cls_full, cls_rust, known_classes, known_classes_full):
    """生成 factory 自由函数（放在 import_lib! 中）"""
    rust_param_decl_parts = []
    sample_args_parts = []
    params = unique_param_names(fm.get("params", []))
    for i, p in enumerate(params):
        rt = map_param(p, known_classes)
        if rt is None:
            return None
        pname = p.get("name") or f"arg{i}"
        rust_param_decl_parts.append(f"{pname}: {rt}")
        sample = sample_value_for_type(rt)
        sample_args_parts.append(sample)

    rt_ret = fm.get("return_type", "")
    rust_ret, ok = map_return(rt_ret.replace("*", "").strip() or rt_ret, known_classes)
    if not ok or rust_ret is None:
        if rt_ret.endswith("*"):
            inner = rt_ret.rstrip("* ").strip()
            if inner in known_classes:
                rust_ret = cpp_class_to_rust(inner)
            else:
                return None
        else:
            return None
    if rust_ret.startswith("*mut ") or rust_ret.startswith("*const "):
        return None

    method_name = cpp_fn_name_to_rust(fm["name"])
    if method_name in ("create", "new", "make"):
        rust_fn = f"{cls_rust.lower()}_new"
    elif method_name in ("create_from", "from"):
        rust_fn = f"{cls_rust.lower()}_from"
    else:
        rust_fn = f"{cls_rust.lower()}_{method_name}"

    return {
        "rust_name": rust_fn,
        "rust_param_decl": ", ".join(rust_param_decl_parts),
        "rust_return": rust_ret,
        "cpp_sig": cpp_full_factory_sig(cls_full, fm, known_classes_full),
        "sample_args": ", ".join(sample_args_parts),
    }


def sample_value_for_type(rust_type):
    """生成测试时传给该类型的示例值"""
    if rust_type in ("i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "isize", "usize"):
        return "0"
    if rust_type in ("f32", "f64"):
        return "0.0"
    if rust_type == "bool":
        return "false"
    if rust_type == "*const i8":
        return 'b"\\0".as_ptr() as *const i8'
    if rust_type == "*mut i8":
        return 'std::ptr::null_mut()'
    if rust_type.startswith("*const "):
        return "std::ptr::null()"
    if rust_type.startswith("*mut "):
        return "std::ptr::null_mut()"
    return "Default::default()"


def build_free_function(fn, known_classes, known_classes_full):
    """生成自由函数上下文"""
    rust_param_decl_parts = []
    sample_args_parts = []
    params = unique_param_names(fn.get("params", []))
    for i, p in enumerate(params):
        rt = map_param(p, known_classes)
        if rt is None:
            return None
        pname = p.get("name") or f"arg{i}"
        rust_param_decl_parts.append(f"{pname}: {rt}")
        sample_args_parts.append(sample_value_for_type(rt))
    rt_ret, ok = map_return(fn.get("return_type", "void"), known_classes)
    if not ok:
        return None
    return {
        "rust_name": cpp_fn_name_to_rust(fn["name"]),
        "rust_param_decl": ", ".join(rust_param_decl_parts),
        "rust_return": rt_ret,
        "cpp_sig": cpp_full_func_sig(fn, known_classes_full),
        "sample_args": ", ".join(sample_args_parts),
    }


def unique_rust_names(funcs):
    """对同名自由函数加后缀（按参数类型）避免 Rust 重名"""
    name_counts = {}
    for f in funcs:
        name_counts[f["rust_name"]] = name_counts.get(f["rust_name"], 0) + 1
    seen = set()
    for f in funcs:
        if name_counts[f["rust_name"]] > 1:
            # 加后缀：_paramtype1_paramtype2
            types = []
            for part in f.get("cpp_sig", "").split("(", 1)[-1].rstrip(")").split(","):
                part = part.strip()
                if not part:
                    continue
                types.append(part.replace(" ", "").replace("*", "ptr").replace("&", "ref"))
            suffix = "_".join(types) if types else "v"
            new_name = f"{f['rust_name']}_{suffix}"
            n = 1
            while new_name in seen:
                new_name = f"{f['rust_name']}_{suffix}_{n}"
                n += 1
            seen.add(new_name)
            f["rust_name"] = new_name
        else:
            seen.add(f["rust_name"])
    return funcs


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--symbols", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--special", required=True)
    args = ap.parse_args()

    symbols = json.loads(Path(args.symbols).read_text())
    feature = symbols["feature"]
    special = load_special(args.special, feature)

    # 在 filter 之前扫描原始 symbols，收集 operator / union 建议包装
    # （filter 会原地修改 symbols，把 operator 方法和 union 类过滤掉）
    suggested_wrappers = []
    pre_known = {c["name"] for c in symbols.get("classes", [])}
    pre_known_full = {c.get("full_name", "") for c in symbols.get("classes", [])}
    for cls in symbols.get("classes", []):
        suggested_wrappers.extend(build_operator_suggestions(cls, pre_known, pre_known_full))
        suggested_wrappers.extend(build_union_suggestions(cls, pre_known))

    filtered = filter_symbols(symbols, special)
    known = set(filtered["known_classes"])
    known_full = {c["full_name"] for c in filtered["classes"]}

    classes_ctx = []
    factory_funcs = []
    free_funcs = []

    for cls in filtered["classes"]:
        ctx = build_method_context(cls, known, known_full)
        classes_ctx.append(ctx)
        factory_funcs.extend(ctx["factory_funcs"])

    for fn in filtered["functions"]:
        ff = build_free_function(fn, known, known_full)
        if ff:
            free_funcs.append(ff)

    # 函数模板：用模板语法实例化（POC 024 验证 hicc-build 接受 max_of<int>(...)）
    template_ffis = build_template_ffis(symbols, special, known)

    # 类模板实例化（POC 验证：using Alias = Stack<int> + 命名空间级 factory）
    cti = build_class_template_instances(symbols, special, known, known_full)
    # 类模板静态方法包装（template_specialization 场景）
    csw = build_template_static_wrappers(symbols, special, known)
    # 变参模板固定参数包装（variadic_template 场景）
    vtw = build_variadic_template_wrappers(symbols, special, known)

    # 注入 extra 类和函数到现有上下文
    classes_ctx.extend(cti["extra_classes"])
    factory_funcs.extend(cti["extra_factory_funcs"])
    free_funcs.extend(csw["extra_free_functions"])
    free_funcs.extend(vtw["extra_free_functions"])

    # 收集所有活跃 C++ 注入（按出现顺序）
    active_cpp_extras = cti["cpp_extras"] + csw["cpp_extras"] + vtw["cpp_extras"]

    # 处理重载（加后缀）
    free_funcs = unique_rust_names(free_funcs)
    factory_funcs = unique_rust_names(factory_funcs)

    render_ctx = {
        "name": feature,
        "special": special,
        "classes": classes_ctx,
        "factory_funcs": factory_funcs,
        "free_functions": free_funcs,
        "template_ffis": template_ffis,
        "suggested_wrappers": suggested_wrappers,
        "active_cpp_extras": active_cpp_extras,
    }

    out = Path(args.out)
    (out / "src").mkdir(parents=True, exist_ok=True)
    (out / "tests").mkdir(parents=True, exist_ok=True)

    (out / "Cargo.toml").write_text(render_template("Cargo.toml.tmpl", render_ctx))
    (out / "build.rs").write_text(render_template("build.rs.tmpl", render_ctx))
    (out / "src/lib.rs").write_text(render_template("lib.rs.tmpl", render_ctx))
    (out / "tests/smoke.rs").write_text(render_template("smoke.rs.tmpl", render_ctx))

    print(f"[rust-gen] {feature}: {len(classes_ctx)} 类, {len(factory_funcs)} factory, {len(free_funcs)} 自由函数, "
          f"{len(template_ffis)} 模板实例 → {out}",
          file=sys.stderr)


if __name__ == "__main__":
    main()
