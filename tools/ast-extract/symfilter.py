#!/usr/bin/env python3
"""symfilter.py - 从 clang AST JSON 提炼项目符号表

输出 symbols.json 结构:
{
  "feature": "<name>",
  "namespaces": ["hicc_usages::xxx", ...],
  "functions": [
    {
      "name": "free_hello",
      "full_name": "hicc_usages::xxx::free_hello",
      "return_type": "void",
      "params": [{"name": "a", "type": "int"}, ...],
      "is_inline": false, "is_static": false, "is_variadic": false,
      "namespace": "hicc_usages::xxx",
      "signature": "void hicc_usages::xxx::free_hello()"
    }
  ],
  "classes": [
    {
      "name": "Greeter",
      "full_name": "hicc_usages::xxx::Greeter",
      "namespace": "hicc_usages::xxx",
      "is_template": false, "is_abstract": false,
      "template_args": [],
      "bases": [{"name": "Base", "access": "public", "is_virtual": false}],
      "methods": [...同 functions...],
      "fields": [{"name": "value", "type": "int", "access": "private"}]
    }
  ],
  "enums": [
    {"name": "Color", "full_name": "...", "underlying_type": "unsigned int",
     "variants": [{"name": "RED", "value": 0}, ...]}
  ]
}
"""
import argparse
import json
import sys
from pathlib import Path

USER_NS_PREFIX = "hicc_usages"


def qual_type(node):
    """提取节点的 qualType 字符串"""
    t = node.get("type")
    if isinstance(t, dict):
        return t.get("qualType", "")
    return ""


def is_implicit(node):
    return bool(node.get("isImplicit", False))


def is_template_function(node):
    """判断 FunctionDecl 是否为模板主定义（clang 会标记 describedTemplate 或其父是 FunctionTemplateDecl）
    """
    # clang AST: 函数模板的 FunctionDecl 含 'describedTemplate' 字段
    if node.get("describedTemplate"):
        return True
    # 或参数是模板参数包 Args...
    for c in node.get("inner", []):
        if c.get("kind") == "ParmVarDecl":
            t = qual_type(c)
            if "..." in t:
                return True
    return False


def parse_params(node):
    """从 FunctionDecl/CXXMethodDecl 的 inner 中提取参数列表"""
    params = []
    for c in node.get("inner", []):
        if c.get("kind") == "ParmVarDecl":
            params.append({
                "name": c.get("name", ""),
                "type": qual_type(c),
                "default_value": c.get("inner", [{}])[0].get("value") if c.get("inner") else None,
            })
    return params


def parse_function(node, namespace):
    """解析 FunctionDecl / CXXMethodDecl"""
    sig = qual_type(node)  # e.g. "void ()" or "Greeter *()"
    # sig 不含函数名，需要自己拼装
    name = node.get("name", "")
    # 推断返回类型和参数类型
    # qualType 格式: "<return type> (<param types>)"
    # 如 "Greeter *()" 表示返回 Greeter* 无参
    # 如 "void (int, double)" 表示返回 void 参数 int,double
    return_type, param_types = split_qual_type(sig)
    params = parse_params(node)
    # 参数类型从 qualType 解析，参数名从 ParmVarDecl 取，按位置对齐
    for i, p in enumerate(params):
        if i < len(param_types):
            p["type"] = param_types[i]

    full_name = f"{namespace}::{name}" if namespace else name
    return {
        "name": name,
        "full_name": full_name,
        "namespace": namespace,
        "return_type": return_type,
        "params": params,
        "signature": f"{return_type} {full_name}({', '.join(p['type'] for p in params)})".strip(),
        "is_inline": bool(node.get("isInline")),
        "is_static": node.get("storageClass") == "static",
        "is_const": False,  # 设置在 method 中
        "is_virtual": bool(node.get("isVirtual")),
        "is_pure": bool(node.get("isPure")),
        "is_variadic": bool(node.get("isVariadic")),
    }


def split_qual_type(sig):
    """从 'Greeter *()' 或 'void (int, double)' 中拆出返回类型和参数类型列表"""
    sig = sig.strip()
    # 找到最外层的括号
    depth = 0
    open_idx = -1
    close_idx = -1
    for i, ch in enumerate(sig):
        if ch == "(":
            if depth == 0 and open_idx == -1:
                open_idx = i
            depth += 1
        elif ch == ")":
            depth -= 1
            if depth == 0:
                close_idx = i
                break
    if open_idx == -1 or close_idx == -1:
        return sig, []
    return_type = sig[:open_idx].strip()
    params_str = sig[open_idx + 1:close_idx].strip()
    if not params_str:
        return return_type, []
    # 拆分参数类型（考虑模板/嵌套括号）
    param_types = split_top_level_commas(params_str)
    return return_type, [p.strip() for p in param_types]


def split_top_level_commas(s):
    """按顶层逗号拆分（考虑 <>, (), [] 嵌套）"""
    parts = []
    depth = 0
    cur = ""
    for ch in s:
        if ch in "<([":
            depth += 1
            cur += ch
        elif ch in ">)]":
            depth -= 1
            cur += ch
        elif ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    if cur.strip():
        parts.append(cur)
    return parts


def parse_method(node, namespace, class_name):
    """解析 CXXMethodDecl"""
    fn = parse_function(node, namespace)
    fn["class"] = class_name
    # clang 把 const/volatile 后缀放在 type.qualType 末尾（如 "void () const"）
    sig = qual_type(node)
    trailing = ""
    for q in (" const volatile", " volatile const", " const", " volatile", " restrict", " &&", " &"):
        if sig.endswith(q):
            trailing = q
            sig = sig[:-len(q)].strip()
            break
    fn["is_const"] = "const" in trailing
    fn["is_volatile"] = "volatile" in trailing
    fn["is_ref_qual_rvalue"] = trailing.endswith("&&")
    fn["is_ref_qual_lvalue"] = trailing.endswith("&") and not fn["is_volatile"]
    # 重新生成 signature 包含限定符
    fn["signature"] = fn["signature"].rstrip(")") + ")" + trailing
    return fn


def parse_field(node):
    """解析 FieldDecl"""
    return {
        "name": node.get("name", ""),
        "type": qual_type(node),
        "access": node.get("access", "private"),
    }


def parse_function_template(node, namespace):
    """解析 FunctionTemplateDecl —— 提取主模板签名（不含实例化）

    实例化类型由 rust_gen 从 special.yaml 读取或默认 [int, double]，
    C++ 编译器对 header 内的模板定义做隐式实例化。
    """
    name = node.get("name", "")
    full_name = f"{namespace}::{name}" if namespace else name
    template_params = []
    primary_sig = None  # 主模板的 FunctionDecl（含 T 类型参数）
    for c in node.get("inner", []):
        k = c.get("kind")
        if k == "TemplateTypeParmDecl":
            template_params.append({
                "name": c.get("name", ""),
                "tagUsed": c.get("tagUsed", "typename"),
            })
        elif k == "FunctionDecl" and primary_sig is None:
            # 第一个 FunctionDecl 是主模板
            primary_sig = c
    if primary_sig is None:
        return None
    fn = parse_function(primary_sig, namespace)
    fn["template_params"] = template_params
    fn["is_template_primary"] = True
    return fn


def parse_class_template(node, namespace):
    """解析 ClassTemplateDecl —— 提取主类签名（不含实例化）

    返回与 parse_class 相同结构，外加 template_params 字段。
    """
    name = node.get("name", "")
    template_params = []
    primary_decl = None
    for c in node.get("inner", []):
        k = c.get("kind")
        if k == "TemplateTypeParmDecl":
            template_params.append({
                "name": c.get("name", ""),
                "tagUsed": c.get("tagUsed", "typename"),
            })
        elif k == "CXXRecordDecl" and primary_decl is None:
            primary_decl = c
    if primary_decl is None:
        return None
    cls = parse_class(primary_decl, namespace)
    cls["template_params"] = template_params
    cls["is_template_primary"] = True
    return cls


def parse_class(node, namespace):
    """解析 CXXRecordDecl"""
    name = node.get("name", "")
    full_name = f"{namespace}::{name}" if namespace else name
    dd = node.get("definitionData", {}) or {}
    is_abstract = bool(dd.get("abstract", False))
    is_union = node.get("tagUsed") == "union"
    bases = []
    methods = []
    fields = []
    for c in node.get("inner", []):
        k = c.get("kind")
        if k == "CXXBaseSpecifier":
            bases.append({
                "name": qual_type(c),
                "access": c.get("access", "public"),
                "is_virtual": bool(c.get("isVirtual")),
            })
        elif k == "CXXMethodDecl":
            if is_implicit(c):
                continue
            methods.append(parse_method(c, namespace, name))
        elif k == "FieldDecl":
            fields.append(parse_field(c))
    return {
        "name": name,
        "full_name": full_name,
        "namespace": namespace,
        "is_template": False,
        "is_abstract": is_abstract,
        "is_union": is_union,
        "template_args": [],
        "bases": bases,
        "methods": methods,
        "fields": fields,
    }


def collect_namespace_ids(node, ns_chain):
    """收集所有 user namespace 下的 NamespaceDecl ID（用于区分真正的命名空间级类 vs 嵌套类）"""
    ids = set()
    def _walk(n, chain):
        if not isinstance(n, dict):
            return
        kind = n.get("kind", "")
        name = n.get("name", "")
        if kind == "NamespaceDecl":
            new_chain = chain + [name] if name else chain
            full_ns = "::".join(new_chain)
            if name and full_ns.startswith(USER_NS_PREFIX):
                if n.get("id"):
                    ids.add(n.get("id"))
            for c in n.get("inner", []):
                _walk(c, new_chain)
        elif kind == "TranslationUnitDecl":
            for c in n.get("inner", []):
                _walk(c, chain)
    _walk(node, ns_chain)
    return ids


def walk(node, ns_chain, acc, parent_record_ids=None):
    """递归遍历 AST，收集 user namespace 下的符号

    ns_chain: 当前命名空间链（list of names）
    acc: 累积 {functions, classes, enums, namespaces}
    parent_record_ids: 当前所在 CXXRecordDecl 的 ID 集合（用于识别 Owner::Impl 这种 namespace 级定义但属于嵌套类的情形）
    """
    if not isinstance(node, dict):
        return
    kind = node.get("kind", "")
    name = node.get("name", "")
    parent_record_ids = parent_record_ids or set()

    # 第一遍：如果还没收集 ns_ids，先扫描一遍
    if "ns_ids" not in acc:
        acc["ns_ids"] = collect_namespace_ids(node, ns_chain)

    if kind == "TranslationUnitDecl":
        for c in node.get("inner", []):
            walk(c, ns_chain, acc)
        return

    # 命名空间：进入后只递归内部
    if kind == "NamespaceDecl":
        if not name or name == "std" or name == "__gnu_cxx" or name.startswith("__"):
            # 跳过系统命名空间，但仍递归 user namespace
            if name and not name.startswith(USER_NS_PREFIX) and name != "hicc_usages":
                return
        new_chain = ns_chain + [name] if name else ns_chain
        full_ns = "::".join(new_chain)
        if name and full_ns.startswith(USER_NS_PREFIX):
            acc["namespaces"].add(full_ns)
        for c in node.get("inner", []):
            walk(c, new_chain, acc)
        return

    # 只处理 user namespace 下的符号
    full_ns = "::".join(ns_chain)
    if not full_ns.startswith(USER_NS_PREFIX):
        # 系统代码不再深入（早剪枝）
        return

    if is_implicit(node):
        return

    if kind == "FunctionDecl":
        # 模板函数主定义（含 describedTemplate 字段）—— 已在 FunctionTemplateDecl 捕获，跳过
        if is_template_function(node):
            return
        acc["functions"].append(parse_function(node, full_ns))
    elif kind == "FunctionTemplateDecl":
        # 函数模板主声明：捕获到 function_templates 列表
        # rust_gen 用模板语法 `max_of<int>(...)` 直接生成 FFI
        tmpl = parse_function_template(node, full_ns)
        if tmpl is not None:
            acc["function_templates"].append(tmpl)
        return
    elif kind == "ClassTemplateDecl":
        # 类模板主声明：捕获到 class_templates 列表
        # 仍递归找嵌套的非模板类（少见）
        tmpl = parse_class_template(node, full_ns)
        if tmpl is not None:
            acc["class_templates"].append(tmpl)
        for c in node.get("inner", []):
            if c.get("kind") == "CXXRecordDecl":
                continue  # 主定义已捕获
            walk(c, ns_chain, acc)
        return
    elif kind == "CXXRecordDecl":
        # 检查是否是嵌套类（parentDeclContextId 不是命名空间 ID，说明是 Owner::Impl 这种）
        parent_ctx = node.get("parentDeclContextId")
        ns_ids = acc.get("ns_ids", set())
        if parent_ctx and parent_ctx not in ns_ids:
            # parentDeclContextId 指向某个 CXXRecordDecl —— 是嵌套类的 namespace 级定义
            return
        if node.get("completeDefinition"):
            acc["classes"].append(parse_class(node, full_ns))
        # 把当前 CXXRecordDecl 的 ID 加入 parent_record_ids，递归子节点
        node_id = node.get("id")
        new_parents = set(parent_record_ids)
        if node_id:
            new_parents.add(node_id)
        for c in node.get("inner", []):
            if c.get("kind") in ("CXXRecordDecl", "ClassTemplateDecl", "ClassTemplateSpecializationDecl"):
                continue
            walk(c, ns_chain, acc, new_parents)
        return
    elif kind == "ClassTemplateSpecializationDecl":
        cls = parse_class(node, full_ns)
        cls["is_template"] = True
        # 模板参数从 name 或专门字段
        cls["template_args"] = extract_template_args(node, name)
        acc["classes"].append(cls)
    elif kind == "EnumDecl":
        if node.get("completeDefinition"):
            acc["enums"].append(parse_enum(node, full_ns))

    # 继续递归子节点
    for c in node.get("inner", []):
        walk(c, ns_chain, acc)


def extract_template_args(node, name):
    """从 ClassTemplateSpecializationDecl 提取模板参数"""
    # name 可能是 "Stack" 或 "Stack<int>"，但 clang 通常把 name 拆为 basename 和 args
    # 尝试从 'name' 字段中解析
    if "<" in name:
        args_str = name[name.index("<") + 1:name.rindex(">")]
        return [a.strip() for a in split_top_level_commas(args_str)]
    # 尝试从 templateArguments 字段
    ta = node.get("templateArguments") or node.get("templateArgs")
    if isinstance(ta, list):
        return [qual_type(a) if isinstance(a, dict) else str(a) for a in ta]
    return []


def parse_enum(node, namespace):
    name = node.get("name", "")
    full_name = f"{namespace}::{name}" if namespace else name
    variants = []
    for c in node.get("inner", []):
        if c.get("kind") == "EnumConstantDecl":
            variants.append({
                "name": c.get("name", ""),
                "value": c.get("value"),
            })
    return {
        "name": name,
        "full_name": full_name,
        "namespace": namespace,
        "underlying_type": qual_type(node) if node.get("type") else "unsigned int",
        "variants": variants,
    }


def dedup(items, key):
    """去重：同 key 保留第一个（声明 vs 定义）"""
    seen = set()
    out = []
    for it in items:
        k = key(it)
        if k in seen:
            continue
        seen.add(k)
        out.append(it)
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--ast", required=True)
    ap.add_argument("--name", required=True)
    ap.add_argument("--out", required=True)
    args = ap.parse_args()

    print(f"[symfilter] 加载 AST: {args.ast} ({Path(args.ast).stat().st_size // 1024} KB)", file=sys.stderr)
    with open(args.ast) as f:
        root = json.load(f)

    acc = {
        "functions": [], "classes": [], "enums": [], "namespaces": set(),
        "function_templates": [], "class_templates": [],
    }
    walk(root, [], acc)

    # 去重
    acc["functions"] = dedup(acc["functions"], lambda f: f["full_name"] + str(len(f["params"])))
    acc["classes"] = dedup(acc["classes"], lambda c: c["full_name"] + str(c["template_args"]))
    acc["enums"] = dedup(acc["enums"], lambda e: e["full_name"])
    acc["function_templates"] = dedup(acc["function_templates"], lambda f: f["full_name"] + str(len(f["params"])))
    acc["class_templates"] = dedup(acc["class_templates"], lambda c: c["full_name"])

    # 按方法签名去重（处理重载：同名 + 不同参数列表都保留）
    for cls in acc["classes"]:
        cls["methods"] = dedup(cls["methods"], lambda m: m["name"] + str(len(m["params"])))
    for cls in acc["class_templates"]:
        cls["methods"] = dedup(cls["methods"], lambda m: m["name"] + str(len(m["params"])))

    out = {
        "feature": args.name,
        "namespaces": sorted(acc["namespaces"]),
        "functions": acc["functions"],
        "classes": acc["classes"],
        "enums": acc["enums"],
        "function_templates": acc["function_templates"],
        "class_templates": acc["class_templates"],
    }
    Path(args.out).write_text(json.dumps(out, indent=2, ensure_ascii=False))
    print(f"[symfilter] {args.name}: {len(out['functions'])} 函数, {len(out['classes'])} 类, "
          f"{len(out['function_templates'])} 函数模板, {len(out['class_templates'])} 类模板, "
          f"{len(out['enums'])} 枚举 → {args.out}", file=sys.stderr)


if __name__ == "__main__":
    main()
