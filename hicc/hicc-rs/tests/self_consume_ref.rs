#![feature(specialization)]
use hicc_rs::export_class;
use hicc_rs::*;

pub struct Container<T>(T);

impl<T> Container<T> {
    fn take(self) -> T {
        self.0
    }
    fn count(&self) -> i32 {
        1
    }
}

#[export_class]
impl<T> Container<T> {
    fn take(self) -> T;
    fn count(&self) -> i32;
}

#[test]
fn test_self_consume_ref() {
    unsafe {
        let v: AbiClass<Container<i32>> = transmute(
            crate::to_abi(Container(42)),
        );
        let cnt: i32 = transmute((v.methods.methods.count)(transmute(&v)));
        assert_eq!(cnt, 1);
        let val: i32 = transmute((v.methods.methods.take)(transmute(v)));
        assert_eq!(val, 42);
    }
}
