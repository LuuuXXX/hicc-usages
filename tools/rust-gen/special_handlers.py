#!/usr/bin/env python3
"""special_handlers.py - 处理需要特殊策略的 C++ 特性

涵盖：
- operator 重载（注释式建议包装）
- union（注释式 ValueBox 建议）
- 函数模板（实例化为具体类型）
- 类模板（活跃注入 typedef + factory）
- 类模板静态方法（特化场景）
- 变参函数模板（固定参数数量包装）

这些函数都从 rust_gen 主流程调用，依赖 mapping 模块和 rust_gen 中的通用辅助。
"""
import re

from mapping import (
    map_type, cpp_class_to_rust, cpp_fn_name_to_rust,
    rust_self_for_method,
)


# === operator 检测：哪些 operator 可生成命名包装 ===
OP_NAME_MAP = {
    "+": "add", "-": "sub", "*": "mul", "/": "div",
    "==": "eq", "!=": "ne", "<": "lt", ">": "gt", "<=": "le", ">=": "ge",
}


def extract_operator_symbol(method_name):
    """从 'operator+' 提取 '+', 不是支持的 operator 返回 None"""
    if not method_name.startswith("operator"):
        return None
    sym = method_name[len("operator"):].strip()
    return sym if sym in OP_NAME_MAP else None


def _to_snake_case(name):
    """CamelCase → snake_case（如 ValueBox → value_box）"""
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", name)
    s = re.sub(r"([a-z\d])([A-Z])", r"\1_\2", s)
    return s.lower()


def build_operator_suggestions(cls, known_classes, known_classes_full):
    """检测类中的 operator 方法，生成注释式包装建议

    返回 list of {cpp_code, rust_code, rust_placement}
    rust_placement: "import_lib" 表示 Rust 绑定放在 import_lib! 块内（函数级）
    """
    cls_full = cls["full_name"]
    cls_short = cpp_class_to_rust(cls_full)
    cls_lower = cls_short.lower()
    if "::" in cls_full:
        ns = cls_full.rsplit("::", 1)[0]
    else:
        ns = ""

    out = []
    for m in cls.get("methods", []):
        op_sym = extract_operator_symbol(m["name"])
        if op_sym is None:
            continue
        op_name = OP_NAME_MAP[op_sym]
        params = m.get("params", [])
        ret = m.get("return_type", "").strip()
        if ret == cls_short or ret == f"{cls_short} ":
            ret_kind = "value"
        elif ret == "bool":
            ret_kind = "bool"
        else:
            continue
        if len(params) != 1:
            continue
        ptype = params[0]["type"].strip()

        m_ref = re.match(r"^const\s+(.+?)\s*&$", ptype)
        is_class_param = False
        scalar_cpp = None
        scalar_rust = None
        if m_ref:
            inner = m_ref.group(1).strip()
            if inner == cls_short:
                is_class_param = True
        if not is_class_param:
            rust_t, _ = map_type(ptype, known_classes)
            if rust_t is not None:
                scalar_cpp = ptype
                scalar_rust = rust_t

        if not is_class_param and scalar_cpp is None:
            continue

        rust_fn = f"{cls_lower}_{op_name}"

        if is_class_param and ret_kind == "value":
            cpp_code = (f"inline {cls_full}* {rust_fn}(const {cls_full}& a, const {cls_full}& b) "
                        f"{{ return new {cls_full}(a {op_sym} b); }}")
            rust_code = (
                f'#[cpp(func = "{cls_full} * {ns}::{rust_fn}(const {cls_full}&, const {cls_full}&)")]\n'
                f"pub fn {rust_fn}(a: &{cls_short}, b: &{cls_short}) -> {cls_short};"
            )
        elif scalar_cpp is not None and ret_kind == "value":
            cpp_code = (f"inline {cls_full}* {rust_fn}(const {cls_full}& a, {scalar_cpp} s) "
                        f"{{ return new {cls_full}(a {op_sym} s); }}")
            rust_code = (
                f'#[cpp(func = "{cls_full} * {ns}::{rust_fn}(const {cls_full}&, {scalar_cpp})")]\n'
                f"pub fn {rust_fn}(a: &{cls_short}, s: {scalar_rust}) -> {cls_short};"
            )
        elif is_class_param and ret_kind == "bool":
            cpp_code = (f"inline bool {rust_fn}(const {cls_full}& a, const {cls_full}& b) "
                        f"{{ return a {op_sym} b; }}")
            rust_code = (
                f'#[cpp(func = "bool {ns}::{rust_fn}(const {cls_full}&, const {cls_full}&)")]\n'
                f"pub fn {rust_fn}(a: &{cls_short}, b: &{cls_short}) -> bool;"
            )
        else:
            continue

        out.append({
            "cpp_code": cpp_code,
            "rust_code": rust_code,
            "rust_placement": "import_lib",
        })
    return out


def build_union_suggestions(cls, known_classes):
    """检测 union 类，生成 ValueBox 包装类注释式建议

    返回 list of {cpp_code, rust_code, rust_placement}
    rust_placement: "top_level" 表示 Rust 绑定是独立的 import_class! 块，放在 import_lib! 之后
    """
    if not cls.get("is_union"):
        return []
    cls_full = cls["full_name"]
    cls_short = cpp_class_to_rust(cls_full)
    if "::" in cls_full:
        ns = cls_full.rsplit("::", 1)[0]
    else:
        ns = ""
    box_short = f"{cls_short}Box"
    box_lower = _to_snake_case(box_short)
    autogen_ns = f"{ns}_autogen"
    box_full = f"{autogen_ns}::{box_short}"

    fields = []
    for fld in cls.get("fields", []):
        fname = fld["name"]
        ftype = fld["type"].strip()
        rt, _ = map_type(ftype, known_classes)
        if rt is None:
            rt = "i32"
        fields.append({"name": fname, "cpp_type": ftype, "rust_type": rt})

    cpp_lines = [f"namespace {autogen_ns} {{",
                 f"    class {box_short} {{",
                 f"    public:"]
    cpp_lines.append(f"        static {box_full}* create() {{ {box_short}* b = new {box_short}; b->u_ = new {cls_full}{{}}; b->tag_ = 0; return b; }}")
    for i, f in enumerate(fields):
        cpp_lines.append(f"        static {box_full}* from_{f['name']}({f['cpp_type']} v) {{ {box_short}* b = create(); b->u_->{f['name']} = v; b->tag_ = {i}; return b; }}")
    cpp_lines.append(f"        static void free({box_full}* self) {{ if (self) {{ delete self->u_; delete self; }} }}")
    for f in fields:
        cpp_lines.append(f"        {f['cpp_type']} {f['name']}() const {{ return u_->{f['name']}; }}")
    cpp_lines.append(f"        int type_tag() const {{ return tag_; }}")
    cpp_lines.append(f"    private:")
    cpp_lines.append(f"        {cls_full}* u_;")
    cpp_lines.append(f"        int tag_;")
    cpp_lines.append(f"    }};")
    cpp_lines.append(f"}}")
    cpp_code = "\n".join(cpp_lines)

    rust_lines = [
        f"hicc::import_class! {{",
        f'    #[cpp(class = "{box_full}", destroy = "{box_full}::free")]',
        f"    pub class {box_short} {{",
    ]
    for f in fields:
        rust_lines.append(f'        #[cpp(method = "{f["cpp_type"]} {f["name"]}() const")]')
        rust_lines.append(f'        pub fn {f["name"]}(&self) -> {f["rust_type"]};')
    rust_lines.append(f"    }}")
    rust_lines.append(f"}}")
    rust_lines.append(f"// 将以下条目添加到 import_lib! 块中：")
    rust_lines.append(f"    pub class {box_short};")
    for i, f in enumerate(fields):
        stripped = f["name"]
        if stripped.startswith("as_"):
            stripped = stripped[3:]
        rust_lines.append(
            f'    #[cpp(func = "{box_full} * {box_full}::from_{f["name"]}({f["cpp_type"]})")]'
        )
        sample = "0" if f["rust_type"] in ("i32", "i64", "u32", "u64", "isize", "usize") else "0.0"
        rust_lines.append(f"    pub fn {box_lower}_from_{stripped}(v: {f['rust_type']}) -> {box_short};  // sample: {sample}")
    rust_code = "\n".join(rust_lines)

    return [{
        "cpp_code": cpp_code,
        "rust_code": rust_code,
        "rust_placement": "top_level",
    }]


def _alias_name(template_short, instantiation_type):
    """根据模板短名和实例化类型生成 alias 名：Stack + int → IntStack"""
    type_map = {
        "int": "Int", "double": "Double", "float": "Float", "long": "Long",
        "char": "Char", "bool": "Bool", "short": "Short", "string": "String",
    }
    prefix = type_map.get(instantiation_type, instantiation_type.capitalize())
    return f"{prefix}{template_short}"


def substitute_template_type(type_str, template_params, instantiation_type):
    """把类型字符串中的模板参数替换为实例化类型"""
    for tp in template_params:
        tn = tp["name"]
        if not tn:
            continue
        type_str = re.sub(rf"\b{re.escape(tn)}\b", instantiation_type, type_str)
    return type_str


def instantiate_function_template(tmpl, instantiation_type, known_classes):
    """实例化一个函数模板，返回 Rust FFI 入口"""
    tmpl_params = tmpl.get("template_params", [])
    full_name = tmpl["full_name"]
    short_name = tmpl["name"]

    from rust_gen import unique_param_names, map_param, map_return, sample_value_for_type

    cpp_param_types = []
    rust_param_decls = []
    sample_args = []
    params = unique_param_names(tmpl.get("params", []))
    for i, p in enumerate(params):
        cpp_t = substitute_template_type(p["type"], tmpl_params, instantiation_type)
        cpp_param_types.append(cpp_t)
        rt, _ = map_type(cpp_t, known_classes)
        if rt is None:
            return None
        pname = p.get("name") or f"arg{i}"
        rust_param_decls.append(f"{pname}: {rt}")
        sample_args.append(sample_value_for_type(rt))

    rt_cpp = substitute_template_type(tmpl.get("return_type", "void"), tmpl_params, instantiation_type)
    rt_rust, _ = map_type(rt_cpp, known_classes)
    if rt_rust is None:
        return None

    type_suffix = instantiation_type.replace(" ", "_")
    rust_name = f"{short_name}_{type_suffix}"

    cpp_args = ", ".join(cpp_param_types)
    cpp_sig = f"{rt_cpp} {full_name}<{instantiation_type}>({cpp_args})"

    return {
        "rust_name": rust_name,
        "rust_param_decl": ", ".join(rust_param_decls),
        "rust_return": rt_rust,
        "cpp_sig": cpp_sig,
        "sample_args": ", ".join(sample_args),
    }


def build_template_ffis(symbols, special, known_classes):
    """为所有函数模板生成实例化 FFI 入口"""
    default_types = ["int", "double"]
    out = []
    for tmpl in symbols.get("function_templates", []):
        types = special.get("template_instantiations", default_types)
        for t in types:
            ffi = instantiate_function_template(tmpl, t, known_classes)
            if ffi is not None:
                out.append(ffi)
    return out


def _instantiate_method(method, T, inst_type, cls_full, known_classes, known_classes_full):
    """把模板方法中的 T 替换为 inst_type，生成 import_class! 方法上下文"""
    from rust_gen import map_param, map_return, unique_param_names, cpp_full_method_sig
    from mapping import cpp_fn_name_to_rust

    name = method.get("name", "")
    if name.startswith("operator") or name.startswith("~"):
        return None
    if method.get("is_static"):
        return None
    for p in method.get("params", []):
        pt = p.get("type", "")
        if T in pt and ("<" in pt or ">" in pt):
            return None

    def subst(t):
        return re.sub(rf"\b{re.escape(T)}\b", inst_type, t)

    ret_cpp = subst(method.get("return_type", "void"))
    if ret_cpp == "()":
        ret_cpp = "void"

    ret_rust, ok = map_return(ret_cpp, known_classes)
    if not ok or ret_rust is None:
        return None

    params = unique_param_names(method.get("params", []))
    rust_param_decl_parts = []
    cpp_param_types = []
    for i, p in enumerate(params):
        ptype = subst(p.get("type", ""))
        rt = map_param({"type": ptype}, known_classes)
        if rt is None:
            return None
        pname = p.get("name") or f"arg{i}"
        rust_param_decl_parts.append(f"{pname}: {rt}")
        cpp_param_types.append(ptype)

    cls_short = cls_full.split("::")[-1]
    suffix = ""
    if method.get("is_const"):
        suffix += " const"
    cpp_method_sig = f"{ret_cpp} {name}({', '.join(cpp_param_types)}){suffix}".strip()

    rust_self = rust_self_for_method(method)
    if rust_self is None:
        return None

    return {
        "rust_name": cpp_fn_name_to_rust(name),
        "rust_self": rust_self,
        "rust_param_decl": ", ".join(rust_param_decl_parts),
        "rust_return": ret_rust,
        "cpp_method_sig": cpp_method_sig,
    }


def _dedup_methods(methods):
    """同名方法只保留第一个"""
    seen = set()
    out = []
    for m in methods:
        if m["rust_name"] in seen:
            continue
        seen.add(m["rust_name"])
        out.append(m)
    return out


def build_class_template_instances(symbols, special, known_classes, known_classes_full):
    """为类模板生成实例化绑定（活跃注入 typedef + 命名空间级 factory）"""
    instantiations_cfg = special.get("class_template_instantiations", {})
    cpp_extras = []
    extra_classes = []
    extra_factory_funcs = []

    for tmpl in symbols.get("class_templates", []):
        tmpl_short = tmpl["name"]
        tmpl_full = tmpl["full_name"]
        if "::" in tmpl_full:
            ns = tmpl_full.rsplit("::", 1)[0]
        else:
            ns = ""

        if tmpl_short not in instantiations_cfg:
            continue
        types = instantiations_cfg.get(tmpl_short, ["int"])
        tmpl_params = [tp["name"] for tp in tmpl.get("template_params", []) if tp.get("name")]
        if len(tmpl_params) != 1:
            continue
        T = tmpl_params[0]
        all_methods = tmpl.get("methods", [])

        for inst_type in types:
            alias_short = _alias_name(tmpl_short, inst_type)
            alias_full = f"{ns}::{alias_short}"
            create_fn = f"create_{_to_snake_case(alias_short)}"
            free_fn = f"free_{_to_snake_case(alias_short)}"

            cpp_lines = [
                f"namespace {ns} {{",
                f"    using {alias_short} = {tmpl_short}<{inst_type}>;",
                f"    inline {alias_full}* {create_fn}() {{ return new {alias_short}(); }}",
                f"    inline void {free_fn}({alias_full}* self) {{ delete self; }}",
                f"}}",
            ]
            cpp_extras.append("\n".join(cpp_lines))

            inst_known = set(known_classes) | {alias_short}
            inst_known_full = set(known_classes_full) | {alias_full}

            methods_ctx = []
            for m in all_methods:
                m_inst = _instantiate_method(m, T, inst_type, alias_full, inst_known, inst_known_full)
                if m_inst is not None:
                    methods_ctx.append(m_inst)
            methods_ctx = _dedup_methods(methods_ctx)

            extra_classes.append({
                "rust_name": alias_short,
                "cpp_full_name": alias_full,
                "destroy": f"{ns}::{free_fn}" if methods_ctx else None,
                "methods": methods_ctx,
                "factory_funcs": [],
            })

            extra_factory_funcs.append({
                "rust_name": _to_snake_case(alias_short) + "_new",
                "rust_param_decl": "",
                "rust_return": alias_short,
                "cpp_sig": f"{alias_full} * {ns}::{create_fn}()",
                "sample_args": "",
            })

    return {
        "cpp_extras": cpp_extras,
        "extra_classes": extra_classes,
        "extra_factory_funcs": extra_factory_funcs,
    }


def build_template_static_wrappers(symbols, special, known_classes):
    """为类模板的静态方法生成命名空间级包装（template_specialization 场景）"""
    from rust_gen import map_return

    cfg = special.get("template_static_wrappers", {})
    cpp_extras = []
    extra_funcs = []

    for tmpl in symbols.get("class_templates", []):
        tmpl_short = tmpl["name"]
        tmpl_full = tmpl["full_name"]
        if tmpl_short not in cfg:
            continue
        if "::" in tmpl_full:
            ns = tmpl_full.rsplit("::", 1)[0]
        else:
            ns = ""

        for entry in cfg[tmpl_short]:
            inst = entry.get("inst")
            if not inst:
                continue
            methods_map = entry.get("methods", {})

            for cpp_method_name, rust_fn_name in methods_map.items():
                m = next((mm for mm in tmpl.get("methods", []) if mm["name"] == cpp_method_name), None)
                if m is None:
                    continue
                ret_cpp = m.get("return_type", "void")
                if ret_cpp == "()":
                    ret_cpp = "void"
                inline_cpp = (f"namespace {ns} {{ "
                              f"inline {ret_cpp} {rust_fn_name}() "
                              f"{{ return {tmpl_short}<{inst}>::{cpp_method_name}(); }} "
                              f"}}")
                cpp_extras.append(inline_cpp)

                ret_rust, ok = map_return(ret_cpp, known_classes)
                if not ok or ret_rust is None:
                    continue
                extra_funcs.append({
                    "rust_name": rust_fn_name,
                    "rust_param_decl": "",
                    "rust_return": ret_rust,
                    "cpp_sig": f"{ret_cpp} {ns}::{rust_fn_name}()",
                    "sample_args": "",
                })

    return {"cpp_extras": cpp_extras, "extra_free_functions": extra_funcs}


def build_variadic_template_wrappers(symbols, special, known_classes):
    """为变参函数模板生成固定参数数量包装"""
    from rust_gen import map_return

    cfg = special.get("variadic_wrappers", {})
    if not cfg:
        return {"cpp_extras": [], "extra_free_functions": []}

    cpp_extras = []
    extra_funcs = []

    for tmpl in symbols.get("function_templates", []):
        fn_name = tmpl["name"]
        if fn_name not in cfg:
            continue
        fn_full = tmpl["full_name"]
        if "::" in fn_full:
            ns = fn_full.rsplit("::", 1)[0]
        else:
            ns = ""

        ret_cpp = tmpl.get("return_type", "void")
        if ret_cpp == "()":
            ret_cpp = "void"
        ret_rust, ok = map_return(ret_cpp, known_classes)
        if not ok or ret_rust is None:
            continue

        param_type = cfg[fn_name].get("param_type", "int") if isinstance(cfg[fn_name], dict) else "int"
        arities = cfg[fn_name].get("arities", []) if isinstance(cfg[fn_name], dict) else cfg[fn_name]

        param_rust, _ = map_type(param_type, known_classes)
        if param_rust is None:
            continue

        for arity_entry in arities:
            if isinstance(arity_entry, dict):
                arity = arity_entry.get("arity")
                rust_name = arity_entry.get("name", f"{fn_name}_{arity}")
            else:
                arity = arity_entry
                rust_name = f"{fn_name}_{arity}"
            if not arity:
                continue

            args_decl = ", ".join([f"{param_type} a{i}" for i in range(arity)])
            call_args = ", ".join([f"a{i}" for i in range(arity)])
            inline_cpp = (f"namespace {ns} {{ "
                          f"inline {ret_cpp} {rust_name}({args_decl}) "
                          f"{{ return {fn_name}({call_args}); }} "
                          f"}}")
            cpp_extras.append(inline_cpp)

            rust_params = ", ".join([f"a{i}: {param_rust}" for i in range(arity)])
            sample = ", ".join(["0"] * arity) if param_rust in ("i32", "i64", "u32", "u64", "isize", "usize") else ", ".join(["0.0"] * arity)
            extra_funcs.append({
                "rust_name": rust_name,
                "rust_param_decl": rust_params,
                "rust_return": ret_rust,
                "cpp_sig": f"{ret_cpp} {ns}::{rust_name}({args_decl})",
                "sample_args": sample,
            })

    return {"cpp_extras": cpp_extras, "extra_free_functions": extra_funcs}
