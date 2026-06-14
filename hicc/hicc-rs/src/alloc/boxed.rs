use crate::{export_class, AbiClass, AbiType, ClassType, IsValue, ValueType};
use alloc::boxed::Box;

#[export_class(in_hicc)]
impl<T> Box<T> {
    fn get(&self) -> &T {
        &**self
    }
    fn get_mut(&mut self) -> &mut T {
        &mut **self
    }
}

impl<T> AbiType for Box<T>
where
    T: ValueType<Value = IsValue> + ClassType,
{
    type Target = Box<T>;
    type InputType = AbiClass<T>;
    type OutputType = AbiClass<T>;
    fn from_abi(src: AbiClass<T>) -> Self::Target {
        src.take_boxed()
    }
    fn into_abi(src: Self::Target) -> AbiClass<T> {
        AbiClass::with_boxed(src)
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use alloc::boxed::Box;

    #[test]
    fn test_i32() {
        unsafe {
            let mut abi_box: AbiClass<Box<i32>> = transmute(crate::to_abi(Box::new(100)));

            let pval: *const i32 = transmute((abi_box.methods.methods.get)(transmute(&abi_box)));
            assert!(!pval.is_null());
            let pval: *mut i32 =
                transmute((abi_box.methods.methods.get_mut)(transmute(&mut abi_box)));
            assert!(!pval.is_null());
        }
    }

    #[test]
    fn test_box_string() {
        use alloc::string::String;
        unsafe {
            // Box<String> 的 Abi 类型是 AbiClass<String>，可通过 String 的 methods 操作
            let mut abi: AbiClass<String> =
                transmute(crate::to_abi(Box::new(String::from("hello"))));
            let len: usize = transmute((abi.methods.methods.len)(transmute(&abi)));
            assert_eq!(len, 5);

            let abi_str_ref: AbiClass<&str> =
                transmute((abi.methods.methods.as_str)(transmute(&abi)));
            let s: &str = crate::from_abi_val(transmute(abi_str_ref));
            assert_eq!(s, "hello");

            (abi.methods.methods.push_str)(transmute(&mut abi), transmute(crate::to_abi(" world")));
            let len: usize = transmute((abi.methods.methods.len)(transmute(&abi)));
            assert_eq!(len, 11);
        }
    }
}
