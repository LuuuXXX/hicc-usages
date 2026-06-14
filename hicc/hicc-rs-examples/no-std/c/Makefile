# ===========================================================================
# Makefile for the no_std hicc-rs C FFI example
#
# Builds the Rust cdylib (libexample_no_std.so), compiles main.c, and runs.
# NOTE: This package is NOT a workspace member (it conflicts with hicc-rs/std
# feature unification). Built separately here.
# ===========================================================================

EXAMPLE_DIR    := $(abspath ..)
WORKSPACE_ROOT := $(abspath ../../../)
CARGO          := cargo
HEADER         := hicc_no_std_demo.h
TARGET         := /tmp/hicc_no_std_test

.PHONY: all clean run

all: $(TARGET)

$(WORKSPACE_ROOT)/target/debug/libexample_no_std.so: FORCE
	RUSTC_BOOTSTRAP=1 $(CARGO) build --manifest-path $(EXAMPLE_DIR)/Cargo.toml

$(TARGET): main.c $(HEADER) \
           $(WORKSPACE_ROOT)/target/debug/libexample_no_std.so
	gcc -o $(TARGET) main.c -I. \
		-L$(WORKSPACE_ROOT)/target/debug -lexample_no_std

run: $(TARGET)
	LD_LIBRARY_PATH=$(WORKSPACE_ROOT)/target/debug $(TARGET)

# Header is hand-written (can't use cbindgen with no_std). Do not delete it.
clean:
	rm -f $(TARGET)

distclean: clean
	rm -f $(HEADER)

FORCE:

.SUFFIXES:
