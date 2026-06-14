// C++ `int add(int, int = 10)` — Rust doesn't have default args, so we expose
// the full arity signature. Callers that want the default must pass 10.

hicc::cpp! {
    #include "default_args.h"
}

hicc::import_lib! {
    #![link_name = "default_args_hicc"]

    #[cpp(func = "int add(int, int)")]
    pub fn add(a: i32, b: i32) -> i32;
}
