# ===========================================================================
# Makefile for the hicc-rs C FFI example
#
# 1. Builds the Rust cdylib (libexample_basic_lib.so)
# 2. Builds hicc-cbindgen (if not already built)
# 3. Runs hicc-cbindgen to generate hicc_demo.h
# 4. Compiles main.c against the cdylib
# 5. Runs the resulting binary
# ===========================================================================

# ---- Paths ----------------------------------------------------------------
WORKSPACE_ROOT     := $(abspath ../../../)
HICC_CBINDGEN      := $(WORKSPACE_ROOT)/target/debug/hicc-cbindgen
CARGO              := cargo

# ---- Targets --------------------------------------------------------------
HEADER := hicc_demo.h
TARGET := /tmp/hicc_test

.PHONY: all clean run

all: $(TARGET)

# 1. Build the Rust cdylib --------------------------------------------------
$(WORKSPACE_ROOT)/target/debug/libexample_basic_lib.so: FORCE
	RUSTC_BOOTSTRAP=1 $(CARGO) build --manifest-path $(WORKSPACE_ROOT)/Cargo.toml \
		-p example-basic_lib

# 2. Build hicc-cbindgen if needed ------------------------------------------
$(HICC_CBINDGEN):
	RUSTC_BOOTSTRAP=1 $(CARGO) build --manifest-path $(WORKSPACE_ROOT)/Cargo.toml \
		-p hicc-cbindgen

# 3. Generate the C header via hicc-cbindgen ---------------------------------
$(HEADER): $(HICC_CBINDGEN) FORCE
	RUSTC_BOOTSTRAP=1 $(HICC_CBINDGEN) -c $(WORKSPACE_ROOT)/hicc-rs-examples/basic_lib > $(HEADER)

# 4. Compile the C program --------------------------------------------------
$(TARGET): main.c $(HEADER) \
           $(WORKSPACE_ROOT)/target/debug/libexample_basic_lib.so
	gcc -o $(TARGET) main.c -I. \
		-L$(WORKSPACE_ROOT)/target/debug -lexample_basic_lib

# 5. Run --------------------------------------------------------------------
run: $(TARGET)
	LD_LIBRARY_PATH=$(WORKSPACE_ROOT)/target/debug $(TARGET)

# ---- Utilities ------------------------------------------------------------
clean:
	rm -f $(TARGET) $(HEADER)

# Force rebuild of the header when the Rust source changes
FORCE:

.SUFFIXES: