#![feature(specialization)]
use hicc_rs::export_class;
use hicc_rs::*;

pub struct RefHolder<'a, T>(&'a T);

impl<'a, T> RefHolder<'a, T> {
    fn get_ref(&self) -> &'a T {
        self.0
    }
    fn as_ptr(&self) -> *const T {
        self.0 as *const T
    }
}

#[export_class]
impl<'a, T> RefHolder<'a, T> {
    fn get_ref(&self) -> &'a T;
    fn as_ptr(&self) -> *const T;
}

#[test]
fn test_lifetime_param() {
    unsafe {
        static VAL: i32 = 42;
        let holder = RefHolder(&VAL);
        let h: RefHolder<'static, i32> = holder;
        let abi: AbiClass<RefHolder<'static, i32>> =
            transmute(crate::to_abi(h));
        let r: &i32 = transmute((abi.methods.methods.get_ref)(transmute(&abi)));
        assert_eq!(*r, 42);
        let p: *const i32 = transmute((abi.methods.methods.as_ptr)(transmute(&abi)));
        assert!(!p.is_null());
        assert_eq!(*p, 42);
    }
}
