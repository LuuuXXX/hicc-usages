use crate::{AbiClass, ClassType};
use alloc::boxed::Box;
use core::mem;
use core::ptr;

pub struct IsClass;
pub struct IsPOD;
pub struct IsValue;
pub struct IsPtr;
pub struct IsMutPtr;
pub struct IsRef;
pub struct IsMut;

pub trait ValueType {
    const N: usize;
    type Result;
    type Type: 'static;
    type Value: 'static;
}

impl<T> ValueType for T {
    default const N: usize = 0;
    default type Result = Self;
    default type Type = IsPOD;
    default type Value = IsValue;
}

impl<T> ValueType for &T
where
    T: ValueType,
{
    const N: usize = T::N + 1;
    type Result = T::Result;
    type Type = T::Type;
    type Value = IsRef;
}

impl<T> ValueType for &mut T
where
    T: ValueType,
{
    const N: usize = T::N + 1;
    type Result = T::Result;
    type Type = T::Type;
    type Value = IsMut;
}

impl<T> ValueType for *const T
where
    T: ValueType,
{
    const N: usize = T::N + 1;
    type Result = T::Result;
    type Type = T::Type;
    type Value = IsPtr;
}

impl<T> ValueType for *mut T
where
    T: ValueType,
{
    const N: usize = T::N + 1;
    type Result = T::Result;
    type Type = T::Type;
    type Value = IsMutPtr;
}

pub trait AbiType {
    type Target;
    type InputType;
    type OutputType;
    fn into_abi(src: Self::Target) -> Self::OutputType;
    fn from_abi(src: Self::InputType) -> Self::Target;
}

impl<T: ValueType> AbiType for T {
    default type Target = <RustType<T, T::Type, T::Value> as AbiType>::Target;
    default type InputType = <RustType<T, T::Type, T::Value> as AbiType>::InputType;
    default type OutputType = <RustType<T, T::Type, T::Value> as AbiType>::OutputType;
    default fn into_abi(src: Self::Target) -> Self::OutputType {
        unsafe {
            transmute(<RustType<T, T::Type, T::Value> as AbiType>::into_abi(
                transmute(src),
            ))
        }
    }
    default fn from_abi(src: Self::InputType) -> Self::Target {
        unsafe {
            transmute(<RustType<T, T::Type, T::Value> as AbiType>::from_abi(
                transmute(src),
            ))
        }
    }
}

pub struct RustType<T, U, V>(T, U, V);

impl<T, U, V> AbiType for RustType<T, U, V> {
    default type Target = T;
    default type InputType = T;
    default type OutputType = T;
    default fn from_abi(_src: Self::InputType) -> Self::Target {
        todo!()
    }
    default fn into_abi(_src: Self::Target) -> Self::OutputType {
        todo!()
    }
}

impl<T: ValueType, V> AbiType for RustType<T, IsPOD, V> {
    type Target = T;
    type InputType = T;
    type OutputType = T;
    fn from_abi(src: Self::InputType) -> Self::Target {
        src
    }
    fn into_abi(src: Self::Target) -> Self::OutputType {
        src
    }
}

impl<T> AbiType for RustType<*const T, IsClass, IsPtr>
where
    T: ValueType,
    T::Result: ClassType,
{
    type Target = *const T;
    type InputType = *mut AbiClass<T::Result>;
    type OutputType = AbiClass<T::Result>;
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from_abi(src: Self::InputType) -> Self::Target {
        if !src.is_null() {
            return unsafe { (*src).this_ptr(T::N).cast() };
        }
        ptr::null()
    }
    fn into_abi(src: Self::Target) -> Self::OutputType {
        AbiClass::with_ptr(src.cast(), T::N)
    }
}

impl<T> AbiType for RustType<*mut T, IsClass, IsMutPtr>
where
    T: ValueType,
    T::Result: ClassType,
{
    type Target = *mut T;
    type InputType = *mut AbiClass<T::Result>;
    type OutputType = AbiClass<T::Result>;
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from_abi(src: Self::InputType) -> Self::Target {
        if !src.is_null() {
            return unsafe { (*src).this_mut_ptr(T::N).cast() };
        }
        ptr::null_mut()
    }
    fn into_abi(src: Self::Target) -> Self::OutputType {
        AbiClass::with_mut_ptr(src.cast(), T::N)
    }
}

impl<T> AbiType for RustType<&T, IsClass, IsRef>
where
    T: ValueType + 'static,
    T::Result: ClassType,
{
    type Target = &'static T;
    type InputType = *mut AbiClass<T::Result>;
    type OutputType = AbiClass<T::Result>;
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from_abi(src: Self::InputType) -> Self::Target {
        if !src.is_null() {
            return unsafe { &*(*src).this_ptr(T::N).cast() };
        }
        panic!("not reference, null pointer");
    }
    fn into_abi(src: Self::Target) -> Self::OutputType {
        AbiClass::with_ptr(ptr::from_ref(src).cast(), T::N)
    }
}

impl<T> AbiType for RustType<&mut T, IsClass, IsMut>
where
    T: ValueType + 'static,
    T::Result: ClassType,
{
    type Target = &'static mut T;
    type InputType = *mut AbiClass<T::Result>;
    type OutputType = AbiClass<T::Result>;
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from_abi(src: Self::InputType) -> Self::Target {
        if !src.is_null() {
            return unsafe { &mut *(*src).this_mut_ptr(T::N).cast() };
        }
        panic!("not mut reference, null pointer");
    }
    fn into_abi(src: Self::Target) -> Self::OutputType {
        AbiClass::with_mut_ptr(ptr::from_mut(src).cast(), T::N)
    }
}

impl<T> AbiType for RustType<T, IsClass, IsValue>
where
    T: ClassType,
{
    type Target = T;
    type InputType = AbiClass<T>;
    type OutputType = AbiClass<T>;
    fn from_abi(src: Self::InputType) -> Self::Target {
        *src.take_boxed()
    }
    fn into_abi(src: Self::Target) -> Self::OutputType {
        AbiClass::with_boxed(Box::new(src))
    }
}

/// Convert value to ABI representation, handling opaque Target transmute.
/// Use this instead of `<T as AbiType>::into_abi(val)` in contexts where
/// the compiler can't normalize `<T as AbiType>::Target` to `T`.
#[inline(always)]
pub fn to_abi<T: AbiType>(val: T) -> T::OutputType {
    unsafe { T::into_abi(transmute::<T, <T as AbiType>::Target>(val)) }
}

/// Convert from ABI representation back to concrete value.
/// Use this instead of `<T as AbiType>::from_abi(transmute(val))` in contexts where
/// the compiler can't normalize `<T as AbiType>::Target` to `T`.
/// Takes the ABI `OutputType` (e.g. return value of an extern "C" function)
/// and returns the concrete Rust type `T`.
#[inline(always)]
pub fn from_abi_val<T: AbiType>(val: T::OutputType) -> T {
    unsafe {
        transmute::<<T as AbiType>::Target, T>(T::from_abi(
            transmute::<T::OutputType, T::InputType>(val),
        ))
    }
}

/// Zero-cost type-system bypass for `<T as AbiType>::Target` → `T` conversion.
///
/// The compiler cannot normalize `<T as AbiType>::Target` to `T` due to
/// specialization defaults, but at monomorphization time both types are
/// identical (all `AbiType` implementations have `Target = T`).
///
/// This function uses `#[inline(always)]` + pointer read to collapse to a
/// type-level identity at the MIR/LLVM level — zero runtime instructions
/// in optimized builds.
#[inline(always)]
pub unsafe fn transmute<IN, OUT>(src: IN) -> OUT {
    // Zero-cost in release: the debug_assert! is compiled away.
    // In debug builds, catches accidental size mismatches.
    debug_assert!(mem::size_of::<OUT>() == mem::size_of::<IN>());
    // SAFETY: Both IN and OUT represent the same type at runtime
    // (all AbiType impls have Target = T), so this pointer read + forget
    // is an identity operation.
    let p = &src as *const IN;
    let target = unsafe { p.cast::<OUT>().read() };
    mem::forget(src);
    target
}
