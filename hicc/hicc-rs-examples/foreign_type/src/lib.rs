#![feature(specialization)]
hicc_rs::foreign!();

use hicc_rs::future::HiccRuntime;
use hicc_rs::*;
use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;

pub struct TokioRuntime {
    rt: tokio::runtime::Runtime,
}

impl TokioRuntime {
    pub fn new() -> Self {
        Self {
            rt: tokio::runtime::Runtime::new().unwrap(),
        }
    }
}

impl HiccRuntime for TokioRuntime {
    fn block_on(&self, f: Pin<&mut dyn Future<Output = ()>>) {
        self.rt.block_on(f);
    }
    fn spawn(&self, f: Pin<&mut dyn Future<Output = ()>>) {
        self.rt.block_on(f);
    }
}

#[export_class(foreign)]
mod classes {
    impl<T> Vec<T> {
        fn push(&mut self, val: T);
        fn len(&self) -> usize;
        fn is_foreign(&self) -> bool { true }
    }

    impl String {
        fn len(&self) -> usize;
        fn as_str(&self) -> &str;
        fn push_str(&mut self, val: &str);
        async fn async_len(&self) -> usize { self.len() }
        fn is_foreign(&self) -> bool { true }
    }
}

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

fn new_string() -> String {
    String::new()
}

fn new_runtime() -> Box<dyn HiccRuntime> {
    Box::new(TokioRuntime::new())
}

async fn make_name(prefix: *const i8) -> String {
    let prefix_str = unsafe {
        std::ffi::CStr::from_ptr(prefix).to_string_lossy().into_owned()
    };
    format!("{} world", prefix_str)
}

fn process_foreign_set(mut s: HashSet<crate::hicc::Foreign<String>>) -> HashSet<crate::hicc::Foreign<String>> {
    s.insert(crate::hicc::Foreign("from_process".to_string()));
    s
}

fn new_set_foreign_string() -> HashSet<crate::hicc::Foreign<String>> {
    HashSet::new()
}

fn make_foreign_string(s: *const i8) -> crate::hicc::Foreign<String> {
    let val = unsafe { std::ffi::CStr::from_ptr(s).to_string_lossy().into_owned() };
    crate::hicc::Foreign(val)
}

#[export_lib(foreign, name = "foreign_type")]
mod ffi {
    use super::*;
    fn push(v: &mut Vec<String>, s: &str) -> String;
    fn new_vec_string() -> Vec<String>;
    fn new_str() -> &'static str;
    fn new_string() -> String;
    fn new_runtime() -> Box<dyn HiccRuntime>;
    async fn make_name(prefix: *const i8) -> String;
    fn process_foreign_set(s: HashSet<crate::hicc::Foreign<String>>) -> HashSet<crate::hicc::Foreign<String>>;
    fn new_set_foreign_string() -> HashSet<crate::hicc::Foreign<String>>;
    fn make_foreign_string(s: *const i8) -> crate::hicc::Foreign<String>;
}

#[cfg(test)]
mod test {
    use crate::hicc::Foreign;
    use std::future::Future;
    use std::mem;

    #[test]
    fn ffi_push() {
        unsafe {
            let lib = super::ffi::foreign_type();
            let ft = &*lib;

            type NewVecStringRet =
                <Foreign<Vec<String>> as ::hicc_rs::AbiType>::OutputType;
            type NewStrRet =
                <Foreign<&'static str> as ::hicc_rs::AbiType>::OutputType;
            type PushArg1 = <Foreign<&'static mut Vec<String>> as ::hicc_rs::AbiType>::InputType;
            type PushArg2 =
                <Foreign<&'static str> as ::hicc_rs::AbiType>::InputType;
            type PushRet = <Foreign<String> as ::hicc_rs::AbiType>::OutputType;

            let mut v_abi: ::hicc_rs::AbiClass<Foreign<Vec<String>>> =
                mem::transmute::<NewVecStringRet, ::hicc_rs::AbiClass<Foreign<Vec<String>>>>(
                    (ft.new_vec_string)(),
                );

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

    /// Test foreign async: make_name returns Box<dyn Future<Output = Foreign<String>>>,
    /// and calling .wait() on it via FFI yields Foreign<String> which unwraps to "hello world".
    #[test]
    fn ffi_async_make_name_wait() {
        unsafe {
            let lib = super::ffi::foreign_type();
            let ft = &*lib;

            // Types for the FFI
            type FutForeignString = Box<dyn Future<Output = Foreign<String>>>;
            type Rt = Box<dyn ::hicc_rs::future::HiccRuntime>;
            type MakeNameArg = <Foreign<*const i8> as ::hicc_rs::AbiType>::InputType;
            type MakeNameRet = <FutForeignString as ::hicc_rs::AbiType>::OutputType;

            // Create a runtime
            type NewRuntimeRet = <Foreign<Rt> as ::hicc_rs::AbiType>::OutputType;
            let rt_abi: ::hicc_rs::AbiClass<Rt> =
                mem::transmute::<NewRuntimeRet, ::hicc_rs::AbiClass<Rt>>((ft.new_runtime)());

            // Create a CStr prefix "hello"
            let prefix = b"hello\0";
            let prefix_ptr = prefix.as_ptr() as *const i8;
            let prefix_arg = mem::transmute::<*const i8, MakeNameArg>(prefix_ptr);

            // Call make_name via FFI — returns Box<dyn Future<Output = Foreign<String>>>
            let future_abi: ::hicc_rs::AbiClass<FutForeignString> =
                mem::transmute::<MakeNameRet, ::hicc_rs::AbiClass<FutForeignString>>(
                    (ft.make_name)(prefix_arg),
                );

            // Call .wait(future, &rt) to block on the future and get Foreign<String>
            // wait returns R = Foreign<String>, which is AbiClass<Foreign<String>>
            type WaitRet = <Foreign<String> as ::hicc_rs::AbiType>::OutputType;
            let result_abi: ::hicc_rs::AbiClass<Foreign<String>> =
                mem::transmute::<WaitRet, ::hicc_rs::AbiClass<Foreign<String>>>(
                    (future_abi.methods.methods.wait)(
                        mem::transmute(future_abi),
                        mem::transmute(&rt_abi),
                    ),
                );

            // Unwrap Foreign<String> -> String
            let rust_string: String = result_abi.take_boxed().0;
            assert_eq!(rust_string, "hello world");
        }
    }

    #[test]
    fn test_process_foreign_set_direct() {
        use crate::hicc::Foreign;
        use std::collections::HashSet;

        let mut set: HashSet<Foreign<String>> = HashSet::new();
        set.insert(Foreign("hello".to_string()));
        set.insert(Foreign("world".to_string()));

        let result = super::process_foreign_set(set);
        assert_eq!(result.len(), 3);
        assert!(result.contains(&Foreign("hello".to_string())));
        assert!(result.contains(&Foreign("world".to_string())));
        assert!(result.contains(&Foreign("from_process".to_string())));
    }

    #[test]
    fn ffi_hashset_contains() {
        use std::collections::HashSet;

        unsafe {
            let lib = super::ffi::foreign_type();
            let ft = &*lib;

            // Reuse type aliases from ffi_push
            type NewVecStringRet =
                <Foreign<Vec<String>> as ::hicc_rs::AbiType>::OutputType;
            type NewStrRet =
                <Foreign<&'static str> as ::hicc_rs::AbiType>::OutputType;
            type PushArg1 = <Foreign<&'static mut Vec<String>> as ::hicc_rs::AbiType>::InputType;
            type PushArg2 =
                <Foreign<&'static str> as ::hicc_rs::AbiType>::InputType;
            type PushRet = <Foreign<String> as ::hicc_rs::AbiType>::OutputType;

            // Get an empty HashSet<Foreign<String>> via FFI
            let hs_ptr = ft.new_set_foreign_string;
            let hs_ptr: unsafe extern "C" fn() -> ::hicc_rs::AbiClass<HashSet<Foreign<String>>> =
                mem::transmute(hs_ptr);
            let mut hs_abi = hs_ptr();

            // Insert "hello" via Vec push then into set
            let mut v_abi: ::hicc_rs::AbiClass<Foreign<Vec<String>>> =
                mem::transmute::<NewVecStringRet, _>((ft.new_vec_string)());
            let s_abi: ::hicc_rs::AbiClass<&'static str> =
                mem::transmute::<NewStrRet, _>((ft.new_str)());
            let val_hello: ::hicc_rs::AbiClass<Foreign<String>> =
                mem::transmute::<PushRet, _>((ft.push)(
                    mem::transmute::<*mut _, PushArg1>(&mut v_abi),
                    mem::transmute(s_abi),
                ));

            let inserted: bool = mem::transmute(
                (hs_abi.methods.methods.insert)(mem::transmute(&mut hs_abi), mem::transmute(val_hello))
            );
            assert!(inserted);

            // Call process_foreign_set — also get its real signature via transmute
            let pf_ptr = ft.process_foreign_set;
            let pf_ptr: unsafe extern "C" fn(::hicc_rs::AbiClass<HashSet<Foreign<String>>>) -> ::hicc_rs::AbiClass<HashSet<Foreign<String>>> =
                mem::transmute(pf_ptr);
            hs_abi = pf_ptr(mem::transmute(hs_abi));

            // Verify len == 2
            let len: usize = mem::transmute(
                (hs_abi.methods.methods.len)(mem::transmute(&hs_abi))
            );
            assert_eq!(len, 2);

            // Verify contains "hello"
            let s2: ::hicc_rs::AbiClass<&'static str> =
                mem::transmute::<NewStrRet, _>((ft.new_str)());
            let check_hello: ::hicc_rs::AbiClass<Foreign<String>> =
                mem::transmute::<PushRet, _>((ft.push)(
                    mem::transmute::<*mut _, PushArg1>(&mut v_abi),
                    mem::transmute(s2),
                ));
            let found_hello: bool = mem::transmute(
                (hs_abi.methods.methods.contains)(mem::transmute(&hs_abi), mem::transmute(&check_hello))
            );
            assert!(found_hello);
            // Values dropped at end of scope — AbiClass Drop frees heap memory
        }
    }
}
