use crate::export_class;
use std::sync::OnceLock;

#[export_class(in_hicc)]
impl<T> OnceLock<T> {
    fn get(&self) -> Option<&T>;
    fn set(&self, val: T) -> Result<(), T>;
    fn into_inner(self) -> Option<T>;
}

#[cfg(test)]
mod test {
    use crate::*;
    use std::sync::OnceLock;

    #[test]
    fn test_once_lock_get_set() {
        unsafe {
            let lock: OnceLock<i32> = OnceLock::new();
            let abi_lock: AbiClass<OnceLock<i32>> = transmute(crate::to_abi(lock));

            let abi_result: AbiClass<Result<(), i32>> = transmute((abi_lock.methods.methods.set)(
                transmute(&abi_lock),
                transmute(42i32),
            ));
            let is_ok: bool = transmute((abi_result.methods.methods.is_ok)(transmute(&abi_result)));
            assert!(is_ok);

            let abi_opt: AbiClass<Option<&i32>> =
                transmute((abi_lock.methods.methods.get)(transmute(&abi_lock)));
            let is_some: bool = transmute((abi_opt.methods.methods.is_none)(transmute(&abi_opt)));
            assert!(!is_some);
            let ptr: *const &i32 = transmute((abi_opt.methods.methods.as_ref)(transmute(&abi_opt)));
            assert!(!ptr.is_null());
            assert_eq!(**ptr, 42);
        }
    }

    #[test]
    fn test_once_lock_double_set() {
        unsafe {
            let lock: OnceLock<i32> = OnceLock::new();
            let abi_lock: AbiClass<OnceLock<i32>> = transmute(crate::to_abi(lock));

            let abi_result: AbiClass<Result<(), i32>> = transmute((abi_lock.methods.methods.set)(
                transmute(&abi_lock),
                transmute(42i32),
            ));
            let is_ok: bool = transmute((abi_result.methods.methods.is_ok)(transmute(&abi_result)));
            assert!(is_ok);

            let abi_result: AbiClass<Result<(), i32>> = transmute((abi_lock.methods.methods.set)(
                transmute(&abi_lock),
                transmute(99i32),
            ));
            let is_err: bool =
                transmute((abi_result.methods.methods.is_err)(transmute(&abi_result)));
            assert!(is_err);
        }
    }

    #[test]
    fn test_once_lock_into_inner() {
        unsafe {
            let lock: OnceLock<i32> = OnceLock::new();
            let _ = lock.set(42i32);
            let abi_lock: AbiClass<OnceLock<i32>> = transmute(crate::to_abi(lock));

            let abi_opt: AbiClass<Option<i32>> =
                transmute((abi_lock.methods.methods.into_inner)(transmute(abi_lock)));
            let is_none: bool = transmute((abi_opt.methods.methods.is_none)(transmute(&abi_opt)));
            assert!(!is_none);
            let val: i32 = transmute((abi_opt.methods.methods.unwrap)(transmute(abi_opt)));
            assert_eq!(val, 42);
        }
    }
}
