WORKSPACE_ROOT     := $(abspath ../../../../)
HICC_CBINDGEN      := $(WORKSPACE_ROOT)/target/debug/hicc-cbindgen
CARGO              := cargo

HEADER := foo_bar_baz.h
TARGET := /tmp/hicc_foo_bar_baz_test

.PHONY: all clean run

all: run

$(WORKSPACE_ROOT)/target/debug/libexample_foo_bar_baz.so: FORCE
	RUSTC_BOOTSTRAP=1 $(CARGO) build --manifest-path $(WORKSPACE_ROOT)/Cargo.toml \
		-p example-foo_bar_baz

$(HICC_CBINDGEN):
	RUSTC_BOOTSTRAP=1 $(CARGO) build --manifest-path $(WORKSPACE_ROOT)/Cargo.toml \
		-p hicc-cbindgen

$(HEADER): $(HICC_CBINDGEN) FORCE
	RUSTC_BOOTSTRAP=1 $(HICC_CBINDGEN) -c $(WORKSPACE_ROOT)/hicc-rs-examples/foo_bar_baz/foo > $(HEADER)

$(TARGET): main.c $(HEADER) \
           $(WORKSPACE_ROOT)/target/debug/libexample_foo_bar_baz.so
	gcc -o $(TARGET) main.c -I. \
		-L$(WORKSPACE_ROOT)/target/debug -lexample_foo_bar_baz

run: $(TARGET)
	LD_LIBRARY_PATH=$(WORKSPACE_ROOT)/target/debug $(TARGET)

clean:
	rm -f $(TARGET) $(HEADER)

FORCE:

.SUFFIXES:
