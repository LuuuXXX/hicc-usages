#!/usr/bin/env python3
"""Filter clang ast-dump JSON to user-defined nodes (those in the project's own .h/.cpp).

Walks the AST with parent file context: a node's location may omit the
`file` field when nested in another decl that already has it, so we
propagate the current file path down the tree. Nodes whose current
file ends with the user's header or cpp basename are kept.

Usage: python3 filter_ast.py <ast.json> <header_basename> <cpp_basename> <out.json>
"""
import json
import sys
from pathlib import Path


def resolve_file(node, parent_file):
    loc = node.get("loc") if isinstance(node, dict) else None
    if isinstance(loc, dict):
        f = loc.get("file")
        if f:
            return f
    return parent_file  # inherit from parent


def short_loc(node):
    loc = node.get("loc") or node.get("range", {}).get("begin", {})
    if isinstance(loc, dict):
        f = loc.get("file") or loc.get("includedFile") or ""
        line = loc.get("line", "?")
        col = loc.get("col", "?")
        return f"{Path(f).name}:{line}:{col}" if f else f"?:{line}:{col}"
    return "<unknown>"


KEY_KINDS = {
    "FunctionDecl", "CXXRecordDecl", "CXXMethodDecl", "ClassTemplateDecl",
    "ClassTemplateSpecializationDecl", "FunctionTemplateDecl", "VarDecl",
    "FieldDecl", "EnumDecl", "EnumConstantDecl", "NamespaceDecl",
    "CXXConstructorDecl", "CXXDestructorDecl", "TypedefDecl", "RecordDecl",
    "TypeAliasDecl", "UsingDirectiveDecl",
}


def walk(node, out, parent_file, header_bn, cpp_bn):
    if isinstance(node, dict):
        cur_file = resolve_file(node, parent_file)
        in_user = cur_file and (
            Path(cur_file).name == header_bn or Path(cur_file).name == cpp_bn
        )
        if in_user and node.get("kind") in KEY_KINDS:
            entry = {
                "kind": node["kind"],
                "name": node.get("name") or node.get("declKind") or "",
                "file": Path(cur_file).name,
                "loc": short_loc(node),
            }
            if node["kind"] in (
                "CXXMethodDecl", "FunctionDecl",
                "CXXConstructorDecl", "CXXDestructorDecl",
                "FunctionTemplateDecl",
            ):
                entry["type"] = node.get("type", {}).get("qualType", "")
            if node["kind"] in ("FieldDecl", "VarDecl"):
                entry["type"] = node.get("type", {}).get("qualType", "")
            if node["kind"] in (
                "CXXRecordDecl", "RecordDecl",
                "ClassTemplateDecl", "ClassTemplateSpecializationDecl",
            ):
                entry["tagUsed"] = node.get("tagUsed", "")
            out.append(entry)
        for v in node.values():
            walk(v, out, cur_file, header_bn, cpp_bn)
    elif isinstance(node, list):
        for v in node:
            walk(v, out, parent_file, header_bn, cpp_bn)


def main():
    ast_path = sys.argv[1]
    header_bn = sys.argv[2]
    cpp_bn = sys.argv[3]
    out_path = sys.argv[4]
    with open(ast_path) as f:
        d = json.load(f)
    out = []
    walk(d, out, "", header_bn, cpp_bn)
    seen = set()
    uniq = []
    for e in out:
        key = (e["kind"], e["name"], e["loc"])
        if key in seen:
            continue
        seen.add(key)
        uniq.append(e)
    with open(out_path, "w") as f:
        json.dump({"header": header_bn, "count": len(uniq), "decls": uniq}, f, indent=2)
    print(f"[filter_ast] {header_bn}: {len(uniq)} user decls → {out_path}")


if __name__ == "__main__":
    main()
