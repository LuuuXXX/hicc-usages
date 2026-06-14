import os
import sys
from setuptools import setup
from distutils.extension import Extension
from Cython.Build import cythonize

_target_dir = os.path.abspath(
    os.path.join(os.path.dirname(__file__), "..", "..", "..", "target", "debug")
)

_lib_path = os.path.join(_target_dir, "libexample_foreign_type.so")
if not os.path.exists(_lib_path):
    print(f"ERROR: Rust library not found at {_lib_path}")
    print("Build it first with:")
    print("  RUSTC_BOOTSTRAP=1 cargo build -p example-foreign_type --features cbindgen")
    sys.exit(1)

setup(
    name="test_foreign_type",
    ext_modules=cythonize(
        [
            Extension(
                "test_foreign_type",
                ["test_foreign_type.pyx"],
                libraries=["example_foreign_type"],
                library_dirs=[_target_dir],
                runtime_library_dirs=[_target_dir],
            )
        ],
        language_level=3,
    ),
)
