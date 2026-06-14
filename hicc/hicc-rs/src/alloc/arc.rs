use crate::export_class;
use alloc::sync::Arc;

#[export_class(in_hicc)]
impl<T> Arc<T> {
    fn get(&self) -> &T {
        &**self
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use alloc::sync::Arc;

    #[test]
    fn test_arc_i32() {
        unsafe {
            let abi_arc: AbiClass<Arc<i32>> =
                transmute(crate::to_abi(Arc::new(42)));
            let val: &i32 = transmute((abi_arc.methods.methods.get)(transmute(&abi_arc)));
            assert_eq!(val, &42);
        }
    }
}
