#![feature(specialization)]
use hicc_rs::export_lib;
use hicc_rs::*;

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn mul(x: i32, y: i32) -> i32 {
    x * y
}

#[export_lib(name = "fn_decl_demo")]
mod lib {
    use super::*;
    fn add(x: i32, y: i32) -> i32;
    fn mul(x: i32, y: i32) -> i32;
}

#[test]
fn test_export_lib_fn_decl() {
    unsafe {
        let lib = lib::fn_decl_demo();
        let result_add: i32 = transmute((lib.add)(
            transmute(crate::to_abi(3)),
            transmute(crate::to_abi(4)),
        ));
        assert_eq!(result_add, 7);
        let result_mul: i32 = transmute((lib.mul)(
            transmute(crate::to_abi(3)),
            transmute(crate::to_abi(4)),
        ));
        assert_eq!(result_mul, 12);
    }
}
