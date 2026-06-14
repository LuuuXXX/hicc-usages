#![feature(specialization)]
use hicc_rs::export_class;
use hicc_rs::*;

pub struct Counter(i32);

impl Counter {
    fn consume_and_increment(mut self) -> i32 {
        self.0 += 1;
        self.0
    }
}

#[export_class]
impl Counter {
    fn consume_and_increment(mut self) -> i32;
}

#[test]
fn test_mut_self() {
    unsafe {
        let v: AbiClass<Counter> = transmute(crate::to_abi(Counter(10)));
        let val: i32 = transmute((v.methods.methods.consume_and_increment)(transmute(v)));
        assert_eq!(val, 11);
    }
}
