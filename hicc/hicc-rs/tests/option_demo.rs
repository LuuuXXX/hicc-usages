#![feature(specialization)]
use hicc_rs::export_class;
use hicc_rs::*;

pub struct MyContainer<T>(T);

impl<T> MyContainer<T> {
    fn is_none(&self) -> bool {
        false
    }
    fn unwrap(self) -> T {
        self.0
    }
    fn as_ref(&self) -> *const T {
        ::core::ptr::null()
    }
}

#[export_class]
impl<T> MyContainer<T> {
    fn is_none(&self) -> bool;
    fn unwrap(self) -> T;
    fn as_ref(&self) -> *const T;
}

#[test]
fn test_option_demo() {
    unsafe {
        let v: AbiClass<MyContainer<i32>> = transmute(crate::to_abi(MyContainer(42)));
        let is_none: bool = transmute((v.methods.methods.is_none)(transmute(&v)));
        assert!(!is_none);
        let val: i32 = transmute((v.methods.methods.unwrap)(transmute(v)));
        assert_eq!(val, 42);
    }
}
