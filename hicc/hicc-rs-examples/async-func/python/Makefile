WORKSPACE_ROOT     := $(abspath ../../../)
HICC_CBINDGEN      := $(WORKSPACE_ROOT)/target/debug/hicc-cbindgen
CARGO              := cargo

.PHONY: all clean run

all: run

async_func.py:
	RUSTC_BOOTSTRAP=1 $(HICC_CBINDGEN) -c $(WORKSPACE_ROOT)/hicc-rs-examples/async-func \
		-l python --lib=libexample_async_func.so -o async_func.py

run: async_func.py
	LD_LIBRARY_PATH=$(WORKSPACE_ROOT)/target/debug python3 main.py

clean:
	rm -f async_func.py