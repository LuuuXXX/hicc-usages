WORKSPACE_ROOT     := $(abspath ../../../)
CARGO              := cargo

.PHONY: all clean run

all: run

run:
	LD_LIBRARY_PATH=$(WORKSPACE_ROOT)/target/debug python3 main.py

clean:
	@echo "hicc_no_std_demo.py is manually maintained (cbindgen does not support no_std)."
	@echo "To reset: git checkout hicc_no_std_demo.py"
