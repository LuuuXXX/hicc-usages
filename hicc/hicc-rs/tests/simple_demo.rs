#![feature(specialization)]
use hicc_rs::export_class;
use hicc_rs::*;

pub struct MyValue(i32);

impl MyValue {
    fn get(&self) -> i32 {
        self.0
    }
}

#[export_class]
impl MyValue {
    fn get(&self) -> i32;
}

#[test]
fn test_simple_demo() {
    unsafe {
        let v: AbiClass<MyValue> = transmute(crate::to_abi(MyValue(42)));
        let val: i32 = transmute((v.methods.methods.get)(transmute(&v)));
        assert_eq!(val, 42);
    }
}
