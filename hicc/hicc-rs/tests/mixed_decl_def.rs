#![feature(specialization)]
use hicc_rs::export_class;
use hicc_rs::*;

pub struct Mixed<T>(T);

impl<T> Mixed<T> {
    fn get(&self) -> &T {
        &self.0
    }
}

// Test: impl block with both fn declarations (`fn ... ;`) and fn definitions (`fn ... { body }`)
// Declarations auto-forward to the method on the actual type.
// Definitions use the custom body logic.
#[export_class]
impl<T> Mixed<T> {
    fn get(&self) -> &T;
    fn get_custom(&self) -> &T {
        &self.0
    }
    fn self_size(&self) -> usize {
        core::mem::size_of::<Self>()
    }
}

pub struct Wrapper<T>(pub T);

#[export_class]
impl<T> Wrapper<T> {
    fn rewrap(self) -> Self {
        Self(self.0)
    }
}

#[test]
fn test_sync_self_rewrap() {
    unsafe {
        let w = Wrapper(42i32);
        let abi: AbiClass<Wrapper<i32>> = transmute(hicc_rs::to_abi(w));
        let result: AbiClass<Wrapper<i32>> =
            transmute((abi.methods.methods.rewrap)(transmute(abi)));
        let boxed = result.take_boxed();
        assert_eq!(boxed.0, 42);
    }
}

#[test]
fn test_mixed_decl_and_def() {
    unsafe {
        let m: Mixed<i32> = Mixed(42);
        let v: AbiClass<Mixed<i32>> = transmute(crate::to_abi(m));
        let val_decl: &i32 = transmute((v.methods.methods.get)(transmute(&v)));
        assert_eq!(*val_decl, 42);
        let val_custom: &i32 = transmute((v.methods.methods.get_custom)(transmute(&v)));
        assert_eq!(*val_custom, 42);
        let sz: usize = transmute((v.methods.methods.self_size)(transmute(&v)));
        assert_eq!(sz, core::mem::size_of::<Mixed<i32>>());
    }
}
