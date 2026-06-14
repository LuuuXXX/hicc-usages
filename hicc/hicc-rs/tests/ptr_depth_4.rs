#![feature(specialization)]
use hicc_rs::export_class;
use hicc_rs::*;

pub struct Container<T>(T);

impl<T> Container<T> {
    fn get_ptr4(&self) -> *const *const *const *const T {
        ::core::ptr::null()
    }
}

#[export_class]
impl<T> Container<T> {
    fn get_ptr4(&self) -> *const *const *const *const T;
}

#[test]
fn test_ptr_depth_4() {
    unsafe {
        let v: AbiClass<Container<i32>> = transmute(
            crate::to_abi(Container(42)),
        );
        let ptr: *const *const *const *const i32 =
            transmute((v.methods.methods.get_ptr4)(transmute(&v)));
        assert!(ptr.is_null());
    }
}
