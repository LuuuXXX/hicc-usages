use crate::{AbiClass, AbiType, ClassType, IsClass, IsMut, IsPOD, ValueType};
use core::ptr::NonNull;

impl<T: 'static> ValueType for NonNull<T>
where
    T: ValueType,
{
    const N: usize = T::N + 1;
    type Type = T::Type;
    type Value = IsMut;
    type Result = T::Result;
}

impl<T> AbiType for NonNull<T>
where
    T: ValueType,
{
    type Target = <NonNullType<T, T::Type> as AbiType>::Target;
    type InputType = <NonNullType<T, T::Type> as AbiType>::InputType;
    type OutputType = <NonNullType<T, T::Type> as AbiType>::OutputType;
    fn from_abi(src: Self::InputType) -> Self::Target {
        <NonNullType<T, T::Type> as AbiType>::from_abi(src)
    }
    fn into_abi(src: Self::Target) -> Self::OutputType {
        <NonNullType<T, T::Type> as AbiType>::into_abi(src)
    }
}

pub struct NonNullType<T, U>(T, U);

impl<T, U> AbiType for NonNullType<T, U> {
    default type Target = NonNull<T>;
    default type InputType = *mut T;
    default type OutputType = *mut T;
    default fn from_abi(_src: Self::InputType) -> Self::Target {
        todo!()
    }
    default fn into_abi(_src: Self::Target) -> Self::OutputType {
        todo!()
    }
}

impl<T> AbiType for NonNullType<T, IsPOD> {
    type Target = NonNull<T>;
    type InputType = *mut T;
    type OutputType = *mut T;
    fn from_abi(src: Self::InputType) -> Self::Target {
        unsafe { NonNull::new_unchecked(src) }
    }
    fn into_abi(src: Self::Target) -> Self::OutputType {
        src.as_ptr()
    }
}

impl<T> AbiType for NonNullType<T, IsClass>
where
    T: ValueType,
    T::Result: ClassType,
{
    type Target = NonNull<T>;
    type InputType = *mut AbiClass<T::Result>;
    type OutputType = AbiClass<T::Result>;
    fn from_abi(src: Self::InputType) -> Self::Target {
        if !src.is_null() {
            return unsafe { NonNull::new_unchecked((*src).this_mut_ptr(T::N).cast()) };
        }
        panic!("NonNull, null pointer");
    }
    fn into_abi(src: Self::Target) -> Self::OutputType {
        AbiClass::with_mut_ptr(src.as_ptr().cast(), T::N)
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use core::ptr::NonNull;

    #[test]
    fn test_nonnull_i32() {
        unsafe {
            let val = 42;
            let nn: NonNull<i32> = NonNull::new_unchecked(&val as *const i32 as *mut i32);

            // NonNull<i32> (POD): into_abi returns *mut i32, from_abi takes *mut i32
            let ptr: *mut i32 = transmute(crate::to_abi(nn));
            assert!(!ptr.is_null());
            assert_eq!(*ptr, 42);

            // InputType/OutputType are opaque, must transmute to/from concrete *mut i32
            let nn2: NonNull<i32> = transmute(<NonNull<i32> as AbiType>::from_abi(transmute(ptr)));
            assert_eq!(nn2.as_ptr(), nn.as_ptr());
            assert_eq!(*nn2.as_ptr(), 42);
        }
    }

    #[test]
    fn test_nonnull_string() {
        use alloc::string::String;
        unsafe {
            let mut s = String::from("hello");
            // NonNull<String> (IsMut) delegates to String methods like &mut String
            let nn: NonNull<String> = NonNull::new_unchecked(&mut s as *mut String);

            let mut abi: AbiClass<String> = transmute(crate::to_abi(nn));

            let len: usize = transmute((abi.methods.methods.len)(transmute(&abi)));
            assert_eq!(len, 5);

            let abi_str_ref: AbiClass<&str> =
                transmute((abi.methods.methods.as_str)(transmute(&abi)));
            let s_ref: &str = crate::from_abi_val(transmute(abi_str_ref));
            assert_eq!(s_ref, "hello");

            (abi.methods.methods.push_str)(transmute(&mut abi), transmute(crate::to_abi(" world")));
            let len2: usize = transmute((abi.methods.methods.len)(transmute(&abi)));
            assert_eq!(len2, 11);

            let raw_ptr: *mut AbiClass<String> = &mut abi as *mut AbiClass<String>;
            // NonNull<String> (Class): from_abi takes *mut AbiClass<String>
            let nn2: NonNull<String> = <NonNull<String> as AbiType>::from_abi(raw_ptr);
            assert_eq!(nn2.as_ptr(), nn.as_ptr());
        }
    }
}
