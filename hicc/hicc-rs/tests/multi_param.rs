#![feature(specialization)]
use hicc_rs::export_class;
use hicc_rs::*;

pub struct Multi<T, U, V>(T, U, V);

impl<T, U, V> Multi<T, U, V> {
    fn get_first(&self) -> *const T {
        &self.0 as *const T
    }
    fn get_second(&self) -> *const U {
        &self.1 as *const U
    }
    fn count(&self) -> i32 {
        3
    }
}

#[export_class]
impl<T, U, V> Multi<T, U, V> {
    fn get_first(&self) -> *const T;
    fn get_second(&self) -> *const U;
    fn count(&self) -> i32;
}

#[test]
fn test_multi_param() {
    unsafe {
        let v: AbiClass<Multi<i32, f64, bool>> = transmute(crate::to_abi(Multi(
            1, 2.0, true,
        )));
        let c: i32 = transmute((v.methods.methods.count)(transmute(&v)));
        assert_eq!(c, 3);
    }
}
