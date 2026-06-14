#![feature(specialization)]
use hicc_rs::export_class;
use hicc_rs::*;

pub struct Foo<T, U, V>(T, U, V);

impl<T, U, V> Foo<T, U, V> {
    fn get_first(&self) -> *const T {
        &self.0 as *const T
    }
}

#[export_class]
impl<T, U, V> Foo<T, U, V> {
    fn get_first(&self) -> *const T;
}

#[test]
fn test_multi_param_unused() {
    unsafe {
        let v: AbiClass<Foo<i32, f64, bool>> = transmute(crate::to_abi(Foo(42, 1.0, true)));
        let ptr: *const i32 = transmute((v.methods.methods.get_first)(transmute(&v)));
        assert!(!ptr.is_null());
        assert_eq!(*ptr, 42);
    }
}
