WORKSPACE_ROOT     := $(abspath ../../../)
HICC_CBINDGEN      := $(WORKSPACE_ROOT)/target/debug/hicc-cbindgen
CARGO              := cargo

.PHONY: all clean run

all: run

hicc_rust_std.py:
	RUSTC_BOOTSTRAP=1 $(HICC_CBINDGEN) -c $(WORKSPACE_ROOT)/hicc-rs-examples/rust-std \
		-l python --lib=libexample_rust_std.so -o hicc_rust_std.py

run: hicc_rust_std.py
	LD_LIBRARY_PATH=$(WORKSPACE_ROOT)/target/debug python3 main.py

clean:
	rm -f hicc_rust_std.py
