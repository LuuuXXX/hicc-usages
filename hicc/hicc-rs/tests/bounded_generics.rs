#![feature(specialization)]
use hicc_rs::export_class;
use hicc_rs::*;

pub struct Inner(i32);

impl Inner {
    fn get(&self) -> i32 {
        self.0
    }
}

#[export_class]
impl Inner {
    fn get(&self) -> i32;
}

// T: ValueType requires ValueType<Type = IsClass>, so T must be a class type.
pub struct BoundedGeneric<T: ValueType, U, V>(T, U, V);

impl<T: ValueType, U, V> BoundedGeneric<T, U, V> {
    fn count(&self) -> i32 {
        3
    }
}

#[export_class]
impl<T: ValueType, U, V> BoundedGeneric<T, U, V> {
    fn count(&self) -> i32;
}

#[test]
fn test_bounded_generics() {
    unsafe {
        let inner = Inner(42);
        let bg = BoundedGeneric(inner, true, 123u64);
        let abi: AbiClass<BoundedGeneric<Inner, bool, u64>> =
            transmute(crate::to_abi(bg));
        let cnt: i32 = transmute((abi.methods.methods.count)(transmute(&abi)));
        assert_eq!(cnt, 3);
    }
}
