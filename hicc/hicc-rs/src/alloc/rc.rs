use crate::export_class;
use alloc::rc::Rc;

#[export_class(in_hicc)]
impl<T> Rc<T> {
    fn get(&self) -> &T {
        &**self
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use alloc::rc::Rc;

    #[test]
    fn test_rc_i32() {
        unsafe {
            let abi_rc: AbiClass<Rc<i32>> =
                transmute(crate::to_abi(Rc::new(42)));
            let val: &i32 = transmute((abi_rc.methods.methods.get)(transmute(&abi_rc)));
            assert_eq!(val, &42);
        }
    }
}
