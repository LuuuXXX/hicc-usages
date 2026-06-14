#![feature(specialization)]
use hicc_rs::export_class;
use hicc_rs::*;

pub struct InnerVal(i32);

impl InnerVal {
    fn get(&self) -> i32 {
        self.0
    }
}

#[export_class]
impl InnerVal {
    fn get(&self) -> i32;
}

// T: ValueType requires ValueType<Type = IsClass>, so T must be a class type.
pub struct BoundedValue<T: ValueType>(T);

impl<T: ValueType> BoundedValue<T> {
    fn count(&self) -> i32 {
        1
    }
}

#[export_class]
impl<T: ValueType> BoundedValue<T> {
    fn count(&self) -> i32;
}

#[test]
fn test_bounded_params() {
    unsafe {
        let inner = InnerVal(99);
        let bv = BoundedValue(inner);
        let abi: AbiClass<BoundedValue<InnerVal>> =
            transmute(crate::to_abi(bv));
        let cnt: i32 = transmute((abi.methods.methods.count)(transmute(&abi)));
        assert_eq!(cnt, 1);
    }
}
