WORKSPACE_ROOT     := $(abspath ../../../)
HICC_CBINDGEN      := $(WORKSPACE_ROOT)/target/debug/hicc-cbindgen
CARGO              := cargo

HEADER := hicc_async_func.h
TARGET := /tmp/hicc_async_func_test

.PHONY: all clean run

all: $(TARGET)

$(WORKSPACE_ROOT)/target/debug/libexample_async_func.so: FORCE
	RUSTC_BOOTSTRAP=1 $(CARGO) build --manifest-path $(WORKSPACE_ROOT)/Cargo.toml \
		-p example-async_func --features cbindgen

$(HICC_CBINDGEN):
	RUSTC_BOOTSTRAP=1 $(CARGO) build --manifest-path $(WORKSPACE_ROOT)/Cargo.toml \
		-p hicc-cbindgen

$(HEADER): $(HICC_CBINDGEN) FORCE
	RUSTC_BOOTSTRAP=1 $(HICC_CBINDGEN) -c $(WORKSPACE_ROOT)/hicc-rs-examples/async-func > $(HEADER)

$(TARGET): main.c $(HEADER) \
           $(WORKSPACE_ROOT)/target/debug/libexample_async_func.so
	gcc -o $(TARGET) main.c -I. \
		-L$(WORKSPACE_ROOT)/target/debug -lexample_async_func

run: $(TARGET)
	LD_LIBRARY_PATH=$(WORKSPACE_ROOT)/target/debug $(TARGET)

clean:
	rm -f $(TARGET) $(HEADER)

FORCE:

.SUFFIXES: