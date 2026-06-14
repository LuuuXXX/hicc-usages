use hicc_rs::*;

#[export_class(foreign)]
mod classes {
    impl<T> Vec<T> {
        fn push(&mut self, val: T);
        fn len(&self) -> usize;
    }

    impl String {
        fn len(&self) -> usize;
        fn as_str(&self) -> &str;
        fn push_str(&mut self, val: &str);
    }
}

#[export_lib(foreign, name = "foreign_type")]
mod ffi {
    fn push(v: &mut Vec<String>, s: &str) -> String {
        v.push(s.to_string());
        s.to_string()
    }

    fn new_vec_string() -> Vec<String> {
        Vec::new()
    }

    fn new_str() -> &'static str {
        "hello"
    }
}

#[cfg(test)]
mod test {
    use crate::hicc::Foreign;
    use std::mem;

    #[test]
    fn ffi_push() {
        unsafe {
            let lib = super::ffi::foreign_type();
            let ft = &*lib;

            type NewVecStringRet = <Foreign<Vec<String>> as ::hicc_rs::AbiType>::OutputType;
            type NewStrRet = <Foreign<&'static str> as ::hicc_rs::AbiType>::OutputType;
            type PushArg1 = <Foreign<&'static mut Vec<String>> as ::hicc_rs::AbiType>::InputType;
            type PushArg2 = <Foreign<&'static str> as ::hicc_rs::AbiType>::InputType;
            type PushRet = <Foreign<String> as ::hicc_rs::AbiType>::OutputType;

            let mut v_abi: ::hicc_rs::AbiClass<Foreign<Vec<String>>> =
                mem::transmute::<NewVecStringRet, ::hicc_rs::AbiClass<Foreign<Vec<String>>>>((ft
                    .new_vec_string)(
                ));

            let s_abi: ::hicc_rs::AbiClass<&'static str> =
                mem::transmute::<NewStrRet, ::hicc_rs::AbiClass<&'static str>>((ft.new_str)());

            let _result_abi: ::hicc_rs::AbiClass<Foreign<String>> =
                mem::transmute::<PushRet, ::hicc_rs::AbiClass<Foreign<String>>>((ft.push)(
                    mem::transmute::<*mut ::hicc_rs::AbiClass<Foreign<Vec<String>>>, PushArg1>(
                        &mut v_abi as *mut ::hicc_rs::AbiClass<Foreign<Vec<String>>>,
                    ),
                    mem::transmute::<::hicc_rs::AbiClass<&'static str>, PushArg2>(s_abi),
                ));
        }
    }
}
