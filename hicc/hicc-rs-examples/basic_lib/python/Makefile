WORKSPACE_ROOT     := $(abspath ../../../)
HICC_CBINDGEN      := $(WORKSPACE_ROOT)/target/debug/hicc-cbindgen
CARGO              := cargo

.PHONY: all clean run

all: run

hicc_demo.py:
	RUSTC_BOOTSTRAP=1 $(HICC_CBINDGEN) -c $(WORKSPACE_ROOT)/hicc-rs-examples/basic_lib \
		-l python --lib=libexample_basic_lib.so -o hicc_demo.py

run: hicc_demo.py
	LD_LIBRARY_PATH=$(WORKSPACE_ROOT)/target/debug python3 main.py

clean:
	rm -f hicc_demo.py
