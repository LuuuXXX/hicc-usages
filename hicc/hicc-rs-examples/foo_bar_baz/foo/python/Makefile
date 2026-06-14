WORKSPACE_ROOT     := $(abspath ../../../../)
HICC_CBINDGEN      := $(WORKSPACE_ROOT)/target/debug/hicc-cbindgen
CARGO              := cargo

.PHONY: all clean run

all: run

foo_bar_baz.py:
	RUSTC_BOOTSTRAP=1 $(HICC_CBINDGEN) -c $(WORKSPACE_ROOT)/hicc-rs-examples/foo_bar_baz/foo \
		-l python --lib=libexample_foo_bar_baz.so -o foo_bar_baz.py

run: foo_bar_baz.py
	LD_LIBRARY_PATH=$(WORKSPACE_ROOT)/target/debug python3 main.py

clean:
	rm -f foo_bar_baz.py
