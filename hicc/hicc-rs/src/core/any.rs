use crate::export_class;

#[export_class(in_hicc)]
impl &dyn core::any::Any {
    fn type_id(&self) -> [u8; 16] {
        unsafe { ::core::mem::transmute((*self).type_id()) }
    }
}

#[export_class(in_hicc)]
impl &mut dyn core::any::Any {
    fn type_id(&self) -> [u8; 16] {
        unsafe { ::core::mem::transmute((*self).type_id()) }
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use core::any::{Any, TypeId};

    struct Foo;

    #[test]
    fn test() {
        unsafe {
            let any: &dyn Any = &Foo;
            let abi_any: AbiClass<&dyn Any> =
                transmute(crate::to_abi(any));
            let abi_id = (abi_any.methods.methods.type_id)(transmute(&abi_any));
            assert_eq!(transmute::<_, TypeId>(abi_id), Foo.type_id());
        }
    }
}
