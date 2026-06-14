RUST_PROJECT_ROOT := $(abspath ../../../)
HICC_CBINDGEN     := $(RUST_PROJECT_ROOT)/target/debug/hicc-cbindgen
CARGO             := cargo

.PHONY: all clean run

all: build

$(RUST_PROJECT_ROOT)/target/debug/libexample_foreign_type.so: FORCE
	RUSTC_BOOTSTRAP=1 $(CARGO) build --manifest-path $(RUST_PROJECT_ROOT)/Cargo.toml \
		-p example-foreign_type --features cbindgen

hicc_foreign_type.pxd: $(RUST_PROJECT_ROOT)/target/debug/libexample_foreign_type.so
	RUSTC_BOOTSTRAP=1 $(HICC_CBINDGEN) -c $(RUST_PROJECT_ROOT)/hicc-rs-examples/foreign_type \
		-l cython -o hicc_foreign_type.pxd

build: hicc_foreign_type.pxd
	python3 setup.py build_ext --inplace

run: build
	LD_LIBRARY_PATH=$(RUST_PROJECT_ROOT)/target/debug python3 -c "import test_foreign_type; test_foreign_type.main()"

clean:
	rm -rf build/ test_foreign_type.c test_foreign_type*.so hicc_foreign_type.pxd hicc_foreign_type.h

distclean: clean

FORCE:

.SUFFIXES: