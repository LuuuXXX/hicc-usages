# ===========================================================================
# Makefile for the hicc-rs Cython FFI example
#
# 1. Builds the Rust cdylib (libexample_basic_lib.so)
# 2. Runs hicc-cbindgen to generate hicc_demo.pxd
# 3. Compiles test_demo.pyx → test_demo*.so via Cython
# 4. Runs the Python test
# ===========================================================================

WORKSPACE_ROOT     := $(abspath ../../../)
HICC_CBINDGEN      := $(WORKSPACE_ROOT)/target/debug/hicc-cbindgen
CARGO              := cargo

.PHONY: all clean run

# ---- Full build: Rust cdylib + .pxd + Cython ------------------------------
all: build

# 1. Build the Rust cdylib ---------------------------------------------------
$(WORKSPACE_ROOT)/target/debug/libexample_basic_lib.so: FORCE
	RUSTC_BOOTSTRAP=1 $(CARGO) build --manifest-path $(WORKSPACE_ROOT)/Cargo.toml \
		-p example-basic_lib

# 2. Generate the Cython .pxd via hicc-cbindgen ------------------------------
hicc_demo.pxd: $(WORKSPACE_ROOT)/target/debug/libexample_basic_lib.so
	RUSTC_BOOTSTRAP=1 $(HICC_CBINDGEN) -c $(WORKSPACE_ROOT)/hicc-rs-examples/basic_lib \
		-l cython -o hicc_demo.pxd

# 3. Build hicc-cbindgen if needed ------------------------------------------
build: hicc_demo.pxd
	python3 setup.py build_ext --inplace

# 5. Run ---------------------------------------------------------------------
run: build
	LD_LIBRARY_PATH=$(WORKSPACE_ROOT)/target/debug python3 -c "import test_demo; test_demo.main()"

# ---- Utilities ------------------------------------------------------------
clean:
	rm -rf build/ test_demo.c test_demo*.so hicc_demo.pxd hicc_demo.h

distclean: clean

FORCE:

.SUFFIXES: