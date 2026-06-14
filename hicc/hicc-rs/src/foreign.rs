//! Declarative macro that enables `foreign` mode for `#[export_class]` / `#[export_lib]`.
//!
//! Downstream crates that need to wrap third-party types (e.g. `Vec<T>`, `String`)
//! must invoke `foreign!()` in their `src/lib.rs`.
//! This creates a `pub(crate) mod hicc` containing `Foreign<T>`,
//! `RustType`, and `AbiType` — the dispatch machinery required by the
//! `foreign` attribute on `#[export_class]` / `#[export_lib]`.

#[macro_export]
macro_rules! foreign {
    () => {
        pub mod hicc {
            use core::ptr;
            use core::fmt;
            use core::ops::{Deref, DerefMut};
            use core::hash::{Hash, Hasher};
            use $crate::{
                AbiClass, AbiType, ClassType, IsClass, IsMut, IsMutPtr, IsPOD, IsPtr, IsRef, IsValue, ValueType,
            };

            #[cfg(feature = "cbindgen")]
            use $crate::{ExportType, TypeRegistry};

            #[repr(transparent)]
            pub struct Foreign<T>(pub(crate) T);

            impl<T> Deref for Foreign<T> {
                type Target = T;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl<T> DerefMut for Foreign<T> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl<T: Hash> Hash for Foreign<T> {
                fn hash<H: Hasher>(&self, state: &mut H) {
                    self.0.hash(state);
                }
            }

            impl<T: PartialEq> PartialEq for Foreign<T> {
                fn eq(&self, other: &Self) -> bool {
                    self.0.eq(&other.0)
                }
            }

            impl<T: Eq> Eq for Foreign<T> {}

            impl<T: Clone> Clone for Foreign<T> {
                fn clone(&self) -> Self {
                    Foreign(self.0.clone())
                }
            }

            impl<T: fmt::Debug> fmt::Debug for Foreign<T> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    self.0.fmt(f)
                }
            }

            impl<T: fmt::Display> fmt::Display for Foreign<T> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    self.0.fmt(f)
                }
            }

            impl<T: PartialOrd> PartialOrd for Foreign<T> {
                fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                    self.0.partial_cmp(&other.0)
                }
            }

            impl<T: Ord> Ord for Foreign<T> {
                fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                    self.0.cmp(&other.0)
                }
            }

            impl<T> ValueType for Foreign<*const T>
            where
                T: ValueType,
                Foreign<T::Result>: ValueType
            {
                const N: usize = T::N + 1;
                type Type = <Foreign<T::Result> as ValueType>::Type;
                type Value = IsPtr;
                type Result = <Foreign<T::Result> as ValueType>::Result;
            }

            impl<T> ValueType for Foreign<*mut T>
            where
                T: ValueType,
                Foreign<T::Result>: ValueType
            {
                const N: usize = T::N + 1;
                type Type = <Foreign<T::Result> as ValueType>::Type;
                type Value = IsMutPtr;
                type Result = <Foreign<T::Result> as ValueType>::Result;
            }

            impl<T> ValueType for Foreign<&T>
            where
                T: ValueType,
                Foreign<T::Result>: ValueType
            {
                const N: usize = T::N + 1;
                type Type = <Foreign<T::Result> as ValueType>::Type;
                type Value = IsRef;
                type Result = <Foreign<T::Result> as ValueType>::Result;
            }

            impl<T> ValueType for Foreign<&mut T>
            where
                T: ValueType,
                Foreign<T::Result>: ValueType
            {
                const N: usize = T::N + 1;
                type Type = <Foreign<T::Result> as ValueType>::Type;
                type Value = IsMut;
                type Result = <Foreign<T::Result> as ValueType>::Result;
            }

#[cfg(feature = "cbindgen")]
            impl<T: ValueType + AbiType> ExportType for Foreign<T>
            where
                T: $crate::ExportType,
                Foreign<T>: ValueType<Type = IsPOD>,
            {
                fn export_name(registry: &mut TypeRegistry) -> String {
                    <T as $crate::ExportType>::export_name(registry)
                }
            }

            impl<T: 'static> AbiType for Foreign<T>
            where
                T: ValueType,
                Foreign<T::Result>: ValueType,
            {
                type Target = <RustType<
                    T,
                    T::Type,
                    <Foreign<T::Result> as ValueType>::Type,
                    T::Value,
                > as AbiType>::Target;
                type InputType = <RustType<
                    T,
                    T::Type,
                    <Foreign<T::Result> as ValueType>::Type,
                    T::Value,
                > as AbiType>::InputType;
                type OutputType = <RustType<
                    T,
                    T::Type,
                    <Foreign<T::Result> as ValueType>::Type,
                    T::Value,
                > as AbiType>::OutputType;
                fn into_abi(src: Self::Target) -> Self::OutputType {
                    <RustType<T, T::Type, <Foreign<T::Result> as ValueType>::Type, T::Value> as AbiType>::into_abi(src)
                }
                fn from_abi(src: Self::InputType) -> Self::Target {
                    <RustType<T, T::Type, <Foreign<T::Result> as ValueType>::Type, T::Value> as AbiType>::from_abi(src)
                }
            }

            pub struct RustType<T, U, V, W>(T, U, V, W);

            impl<T, U, V, W> AbiType for RustType<T, U, V, W> {
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

            impl<T, W> AbiType for RustType<T, IsPOD, IsPOD, W> {
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

            impl<T, W> AbiType for RustType<T, IsClass, IsPOD, W>
            where
                T: ValueType,
                T::Result: ClassType,
            {
                type Target = <T as AbiType>::Target;
                type InputType = <T as AbiType>::InputType;
                type OutputType = <T as AbiType>::OutputType;
                fn from_abi(src: Self::InputType) -> Self::Target {
                    <T as AbiType>::from_abi(src)
                }
                fn into_abi(src: Self::Target) -> Self::OutputType {
                    <T as AbiType>::into_abi(src)
                }
            }



            // ---- Child (specific) impls: use Foreign<T> class type for #[export_class(foreign)] types ----

            impl<T, U> AbiType for RustType<T, U, IsClass, IsValue>
            where
                Foreign<T>: ClassType,
            {
                type Target = T;
                type InputType = AbiClass<Foreign<T>>;
                type OutputType = AbiClass<Foreign<T>>;
                fn from_abi(src: Self::InputType) -> Self::Target {
                    (*src.take_boxed()).0
                }
                fn into_abi(src: Self::Target) -> Self::OutputType {
                    AbiClass::with_boxed(Box::new(Foreign(src)))
                }
            }

            impl<T, U> AbiType for RustType<*const T, U, IsClass, IsPtr>
            where
                T: ValueType,
                Foreign<T::Result>: ClassType,
            {
                type Target = *const T;
                type InputType = *mut AbiClass<Foreign<T::Result>>;
                type OutputType = AbiClass<Foreign<T::Result>>;
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

            impl<T, U> AbiType for RustType<*mut T, U, IsClass, IsMutPtr>
            where
                T: ValueType,
                Foreign<T::Result>: ClassType,
            {
                type Target = *mut T;
                type InputType = *mut AbiClass<Foreign<T::Result>>;
                type OutputType = AbiClass<Foreign<T::Result>>;
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

            impl<T, U> AbiType for RustType<&T, U, IsClass, IsRef>
            where
                T: ValueType + 'static,
                Foreign<T::Result>: ClassType,
            {
                type Target = &'static T;
                type InputType = *mut AbiClass<Foreign<T::Result>>;
                type OutputType = AbiClass<Foreign<T::Result>>;
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

            impl<T, U> AbiType for RustType<&mut T, U, IsClass, IsMut>
            where
                T: ValueType + 'static,
                Foreign<T::Result>: ClassType,
            {
                type Target = &'static mut T;
                type InputType = *mut AbiClass<Foreign<T::Result>>;
                type OutputType = AbiClass<Foreign<T::Result>>;
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
        }
    };
}
