RUST_PROJECT_ROOT := $(abspath ../../../)
HICC_CBINDGEN     := $(RUST_PROJECT_ROOT)/target/debug/hicc-cbindgen
CARGO             := cargo

HEADER := hicc_rust_std.h
TARGET := /tmp/hicc_rust_std_test

.PHONY: all clean run

all: $(TARGET)

$(RUST_PROJECT_ROOT)/target/debug/libexample_rust_std.so: FORCE
	RUSTC_BOOTSTRAP=1 $(CARGO) build --manifest-path $(RUST_PROJECT_ROOT)/Cargo.toml \
		-p example-rust_std --features cbindgen

$(HEADER): FORCE
	RUSTC_BOOTSTRAP=1 $(HICC_CBINDGEN) -c $(RUST_PROJECT_ROOT)/hicc-rs-examples/rust-std > $(HEADER)

$(TARGET): main.c $(HEADER) \
           $(RUST_PROJECT_ROOT)/target/debug/libexample_rust_std.so
	gcc -o $(TARGET) main.c -I. \
		-L$(RUST_PROJECT_ROOT)/target/debug -lexample_rust_std

run: $(TARGET)
	LD_LIBRARY_PATH=$(RUST_PROJECT_ROOT)/target/debug $(TARGET)

clean:
	rm -f $(TARGET) $(HEADER)

FORCE:

.SUFFIXES: