"""Python ctypes bindings for the no_std hicc-rs cdylib.

Auto-generated target: this file is manually written because
hicc-cbindgen requires the `cbindgen` feature which pulls in `std`,
conflicting with `#![no_std]`.
"""
import ctypes
import os
import sys

_lib_path = os.path.abspath(os.path.join(
    os.path.dirname(__file__), "..", "..", "..", "target", "debug",
    "libexample_no_std.so",
))
if not os.path.exists(_lib_path):
    print(f"Error: cdylib not found at {_lib_path}", file=sys.stderr)
    sys.exit(1)

_lib = ctypes.cdll.LoadLibrary(_lib_path)

# ── AbiMethods struct forward declarations ──────────────────────────
class AbiMethods_Container_i32(ctypes.Structure):
    pass
class AbiMethods_Option_i32(ctypes.Structure):
    pass

# ── AbiClass structs ────────────────────────────────────────────────
class AbiClass_Container_i32(ctypes.Structure):
    _fields_ = [
        ("methods", ctypes.POINTER(AbiMethods_Container_i32)),
        ("this_", ctypes.c_void_p),
        ("level", ctypes.c_size_t),
    ]

class AbiClass_Option_i32(ctypes.Structure):
    _fields_ = [
        ("methods", ctypes.POINTER(AbiMethods_Option_i32)),
        ("this_", ctypes.c_void_p),
        ("level", ctypes.c_size_t),
    ]

# ── AbiMethods _fields_ ─────────────────────────────────────────────
# ---- AbiMethods_Container_i32 ----
AbiMethods_Container_i32._fields_ = [
    ("hicc_destroy", ctypes.CFUNCTYPE(None, AbiClass_Container_i32)),
    ("hicc_make_unique", ctypes.CFUNCTYPE(AbiClass_Container_i32, AbiClass_Container_i32)),
    ("hicc_make_ref_mut", ctypes.CFUNCTYPE(AbiClass_Container_i32, ctypes.POINTER(AbiClass_Container_i32))),
    ("hicc_size_of", ctypes.CFUNCTYPE(ctypes.c_size_t)),
    ("hicc_write", ctypes.CFUNCTYPE(None, ctypes.POINTER(AbiClass_Container_i32), AbiClass_Container_i32)),
    ("hicc_make_ref", ctypes.CFUNCTYPE(AbiClass_Container_i32, ctypes.POINTER(AbiClass_Container_i32))),
    ("get", ctypes.CFUNCTYPE(ctypes.POINTER(ctypes.c_int32), ctypes.POINTER(AbiClass_Container_i32))),
]

# ---- AbiMethods_Option_i32 ----
AbiMethods_Option_i32._fields_ = [
    ("hicc_destroy", ctypes.CFUNCTYPE(None, AbiClass_Option_i32)),
    ("hicc_make_unique", ctypes.CFUNCTYPE(AbiClass_Option_i32, AbiClass_Option_i32)),
    ("hicc_make_ref_mut", ctypes.CFUNCTYPE(AbiClass_Option_i32, ctypes.POINTER(AbiClass_Option_i32))),
    ("hicc_size_of", ctypes.CFUNCTYPE(ctypes.c_size_t)),
    ("hicc_write", ctypes.CFUNCTYPE(None, ctypes.POINTER(AbiClass_Option_i32), AbiClass_Option_i32)),
    ("hicc_make_ref", ctypes.CFUNCTYPE(AbiClass_Option_i32, ctypes.POINTER(AbiClass_Option_i32))),
    ("is_none", ctypes.CFUNCTYPE(ctypes.c_bool, ctypes.POINTER(AbiClass_Option_i32))),
    ("unwrap", ctypes.CFUNCTYPE(ctypes.c_int32, AbiClass_Option_i32)),
    ("take", ctypes.CFUNCTYPE(AbiClass_Option_i32, ctypes.POINTER(AbiClass_Option_i32))),
    ("as_ref", ctypes.CFUNCTYPE(ctypes.POINTER(ctypes.c_int32), ctypes.POINTER(AbiClass_Option_i32))),
    ("as_mut", ctypes.CFUNCTYPE(ctypes.POINTER(ctypes.c_int32), ctypes.POINTER(AbiClass_Option_i32))),
]

# ── Function table ──────────────────────────────────────────────────
class Hicc_no_std_demo(ctypes.Structure):
    _fields_ = [
        ("add", ctypes.CFUNCTYPE(ctypes.c_int32, ctypes.c_int32, ctypes.c_int32)),
        ("negate", ctypes.CFUNCTYPE(ctypes.c_int32, ctypes.c_int32)),
        ("container_value", ctypes.CFUNCTYPE(ctypes.c_int32, AbiClass_Container_i32)),
        ("new_container", ctypes.CFUNCTYPE(AbiClass_Container_i32, ctypes.c_int32)),
        ("double_option", ctypes.CFUNCTYPE(ctypes.c_int64, AbiClass_Option_i32)),
        ("new_option", ctypes.CFUNCTYPE(AbiClass_Option_i32, ctypes.c_int32)),
    ]

# ── Entry point ─────────────────────────────────────────────────────
_lib.no_std_demo.restype = ctypes.POINTER(Hicc_no_std_demo)
_fn_no_std_demo = _lib.no_std_demo()

# ── RAII wrappers ───────────────────────────────────────────────────
class Container_i32:
    def __init__(self, inner: AbiClass_Container_i32):
        self._inner = inner

    def destroy(self):
        if hasattr(self, '_inner') and self._inner is not None:
            _inner = self._inner
            self._inner = None
            _inner.methods[0].hicc_destroy(_inner)

    def get(self):
        _ptr = self._inner.methods[0].get(ctypes.byref(self._inner))
        if _ptr:
            return _ptr[0]
        return None


class Option_i32:
    def __init__(self, inner: AbiClass_Option_i32):
        self._inner = inner

    def destroy(self):
        if hasattr(self, '_inner') and self._inner is not None:
            _inner = self._inner
            self._inner = None
            _inner.methods[0].hicc_destroy(_inner)

    def is_none(self):
        return self._inner.methods[0].is_none(ctypes.byref(self._inner))

    def unwrap(self):
        _ret = self._inner.methods[0].unwrap(self._inner)
        self._inner = None
        return _ret

    def take(self):
        _inner = self._inner.methods[0].take(ctypes.byref(self._inner))
        return Option_i32(_inner)

    def as_ref(self):
        _ptr = self._inner.methods[0].as_ref(ctypes.byref(self._inner))
        if _ptr:
            return _ptr[0]
        return None

    def as_mut(self):
        _ptr = self._inner.methods[0].as_mut(ctypes.byref(self._inner))
        if _ptr:
            return _ptr[0]
        return None

# ── Factory functions ───────────────────────────────────────────────
def add(arg0: int, arg1: int):
    return _fn_no_std_demo.contents.add(ctypes.c_int32(arg0), ctypes.c_int32(arg1))

def negate(arg0: int):
    return _fn_no_std_demo.contents.negate(ctypes.c_int32(arg0))

def container_value(arg0):
    return _fn_no_std_demo.contents.container_value(arg0)

def new_container(arg0: int):
    _inner = _fn_no_std_demo.contents.new_container(ctypes.c_int32(arg0))
    return Container_i32(_inner)

def double_option(arg0):
    return _fn_no_std_demo.contents.double_option(arg0)

def new_option(arg0: int):
    _inner = _fn_no_std_demo.contents.new_option(ctypes.c_int32(arg0))
    return Option_i32(_inner)
