import os
import sys
from setuptools import setup
from distutils.extension import Extension
from Cython.Build import cythonize

# Absolute path to the Rust cdylib build directory
_target_dir = os.path.abspath(
    os.path.join(os.path.dirname(__file__), "..", "..", "..", "target", "debug")
)

# Ensure the Rust library has been built
_lib_path = os.path.join(_target_dir, "libexample_basic_lib.so")
if not os.path.exists(_lib_path):
    print(f"ERROR: Rust library not found at {_lib_path}")
    print("Build it first with:")
    print("  RUSTC_BOOTSTRAP=1 cargo build -p example-basic_lib")
    sys.exit(1)

setup(
    name="test_demo",
    ext_modules=cythonize(
        [
            Extension(
                "test_demo",
                ["test_demo.pyx"],
                libraries=["example_basic_lib"],
                library_dirs=[_target_dir],
                runtime_library_dirs=[_target_dir],
            )
        ],
        language_level=3,
    ),
)
