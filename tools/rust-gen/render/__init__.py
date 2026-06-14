#!/usr/bin/env python3
"""render/__init__.py - Jinja2 模板渲染辅助"""
from pathlib import Path
from jinja2 import Environment, FileSystemLoader

TMPL_DIR = Path(__file__).parent


def render_template(tmpl_name, ctx):
    """渲染 render/<tmpl_name>，返回字符串"""
    env = Environment(
        loader=FileSystemLoader(str(TMPL_DIR)),
        trim_blocks=True,
        lstrip_blocks=True,
        keep_trailing_newline=True,
    )
    tmpl = env.get_template(tmpl_name)
    return tmpl.render(**ctx)
