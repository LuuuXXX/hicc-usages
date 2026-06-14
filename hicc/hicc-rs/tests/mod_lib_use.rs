#![feature(specialization)]
use hicc_rs::export_lib;
use hicc_rs::*;

pub mod tests {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    pub fn negate(x: i32) -> i32 {
        -x
    }
}

#[export_lib(name = "lib_use_demo")]
mod lib {
    use super::tests::*;
    fn add(x: i32, y: i32) -> i32;
    fn negate(x: i32) -> i32;
}

#[test]
fn test_export_lib_with_use() {
    unsafe {
        let lib = lib::lib_use_demo();
        let result_add: i32 = transmute((lib.add)(
            transmute(crate::to_abi(3)),
            transmute(crate::to_abi(4)),
        ));
        assert_eq!(result_add, 7);
        let result_neg: i32 =
            transmute((lib.negate)(transmute(crate::to_abi(5))));
        assert_eq!(result_neg, -5);
    }
}
