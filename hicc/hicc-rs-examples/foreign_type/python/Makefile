WORKSPACE_ROOT     := $(abspath ../../../)
HICC_CBINDGEN      := $(WORKSPACE_ROOT)/target/debug/hicc-cbindgen
CARGO              := cargo

.PHONY: all clean run

all: run

hicc_foreign_type.py:
	RUSTC_BOOTSTRAP=1 $(HICC_CBINDGEN) -c $(WORKSPACE_ROOT)/hicc-rs-examples/foreign_type \
		-l python --lib=libexample_foreign_type.so -o hicc_foreign_type.py

run: hicc_foreign_type.py
	LD_LIBRARY_PATH=$(WORKSPACE_ROOT)/target/debug python3 main.py

clean:
	rm -f hicc_foreign_type.py
