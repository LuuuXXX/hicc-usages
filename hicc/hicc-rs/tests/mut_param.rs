#![feature(specialization)]
use hicc_rs::export_class;
use hicc_rs::*;

pub struct Accumulator(i32);

#[export_class]
impl Accumulator {
    fn add(&mut self, mut delta: i32) -> i32 {
        delta += 100;
        self.0 += delta;
        self.0
    }
}

#[test]
fn test_mut_param() {
    unsafe {
        let mut v: AbiClass<Accumulator> = transmute(to_abi::<Accumulator>(Accumulator(0)));
        let result: i32 = transmute((v.methods.methods.add)(transmute(&mut v), transmute(1)));
        assert_eq!(result, 101);
    }
}
