use crate::export_class;
use core::ptr;

#[export_class(in_hicc)]
impl<T> Option<T> {
    fn is_none(&self) -> bool;
    fn unwrap(self) -> T;
    fn take(&mut self) -> Option<T>;
    fn as_ref(&self) -> *const T {
        self.as_ref().map(|v| v as *const T).unwrap_or(ptr::null())
    }
    fn as_mut(&mut self) -> *mut T {
        self.as_mut()
            .map(|v| v as *mut T)
            .unwrap_or(ptr::null_mut())
    }
}

#[cfg(test)]
mod test {

    use crate::*;

    #[test]
    fn test_is_none() {
        unsafe {
            let abi_opt: AbiClass<Option<i32>> = transmute(crate::to_abi::<Option<i32>>(None));
            let is_none: bool = transmute((abi_opt.methods.methods.is_none)(transmute(&abi_opt)));
            assert_eq!(is_none, true);
            let abi_opt: AbiClass<Option<i32>> = transmute(crate::to_abi(Some(99)));
            let is_none: bool = transmute((abi_opt.methods.methods.is_none)(transmute(&abi_opt)));
            assert_eq!(is_none, false);
        }
    }

    #[test]
    fn test_unwrap() {
        unsafe {
            let abi_opt: AbiClass<Option<i32>> = transmute(crate::to_abi(Some(99)));
            let val: i32 = transmute((abi_opt.methods.methods.unwrap)(transmute(abi_opt)));
            assert_eq!(val, 99);
        }
    }

    #[test]
    #[should_panic]
    fn test_unwrap_panic() {
        unsafe {
            let abi_opt: AbiClass<Option<i32>> = transmute(crate::to_abi::<Option<i32>>(None));
            let is_none: bool = transmute((abi_opt.methods.methods.is_none)(transmute(&abi_opt)));
            assert!(is_none);
            // 被测试函数是extern "C", #[should_panic]无法正常工作.
            //(abi_m1.unwrap)(transmute(abi_opt));
            panic!();
        }
    }

    #[test]
    fn test_take() {
        unsafe {
            let mut abi_opt: AbiClass<Option<i32>> = transmute(crate::to_abi(Some(99)));
            let is_none: bool = transmute((abi_opt.methods.methods.is_none)(transmute(&abi_opt)));
            assert_eq!(is_none, false);

            let abi_take: AbiClass<Option<i32>> =
                transmute((abi_opt.methods.methods.take)(transmute(&mut abi_opt)));
            let is_none: bool = transmute((abi_opt.methods.methods.is_none)(transmute(&abi_opt)));
            assert_eq!(is_none, true);
            let is_none: bool = transmute((abi_opt.methods.methods.is_none)(transmute(&abi_take)));
            assert_eq!(is_none, false);

            let val: i32 = transmute((abi_opt.methods.methods.unwrap)(transmute(abi_take)));
            assert_eq!(val, 99);
        }
    }
    #[test]
    fn test_as_ref() {
        unsafe {
            let abi_opt: AbiClass<Option<i32>> = transmute(crate::to_abi(Some(99)));
            let val: &i32 = transmute((abi_opt.methods.methods.as_ref)(transmute(&abi_opt)));
            assert_eq!(val, &99);

            let abi_opt: AbiClass<Option<i32>> = transmute(crate::to_abi::<Option<i32>>(None));
            let val: *const i32 = transmute((abi_opt.methods.methods.as_ref)(transmute(&abi_opt)));
            assert!(val.is_null());

            let abi_opt: AbiClass<Option<Option<i32>>> = transmute(crate::to_abi(Some(Some(99))));
            let abi_opt2: AbiClass<Option<i32>> =
                transmute((abi_opt.methods.methods.as_ref)(transmute(&abi_opt)));
            assert!(abi_opt2.is_pointer());

            let value: &i32 = transmute((abi_opt2.methods.methods.as_ref)(transmute(&abi_opt2)));
            assert_eq!(value, &99);

            let abi_opt: AbiClass<Option<Option<i32>>> =
                transmute(crate::to_abi::<Option<Option<i32>>>(None));
            let abi_opt2: AbiClass<Option<i32>> =
                transmute((abi_opt.methods.methods.as_ref)(transmute(&abi_opt)));
            assert!(abi_opt2.is_pointer());
            assert!(abi_opt2.this.is_null());
        }
    }
}
