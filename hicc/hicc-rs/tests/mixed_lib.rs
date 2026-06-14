#![feature(specialization)]
use hicc_rs::export_lib;
use hicc_rs::*;

fn sub(x: i32, y: i32) -> i32 {
    x - y
}

#[export_lib(name = "mixed_lib")]
mod lib {
    use super::sub;
    fn sub(x: i32, y: i32) -> i32;
    fn double(x: i32) -> i32 {
        x * 2
    }
}

#[test]
fn test_export_lib_mixed() {
    unsafe {
        let lib = lib::mixed_lib();
        let result_sub: i32 = transmute((lib.sub)(
            transmute(crate::to_abi(10)),
            transmute(crate::to_abi(3)),
        ));
        assert_eq!(result_sub, 7);
        let result_double: i32 =
            transmute((lib.double)(transmute(crate::to_abi(5))));
        assert_eq!(result_double, 10);
    }
}
