#![feature(specialization)]

use core::task::{Context, RawWaker, RawWakerVTable, Waker};
use hicc_rs::export_class;
use hicc_rs::export_lib;
use hicc_rs::future::HiccRuntime;
use hicc_rs::ValueType;
use std::future::Future;
use std::pin::Pin;

pub struct SpinRuntime;

const NOOP_VTABLE: RawWakerVTable = RawWakerVTable::new(
    |p| RawWaker::new(p, &NOOP_VTABLE),
    |_| {},
    |_| {},
    |_| {},
);

fn noop_waker() -> Waker {
    let raw = RawWaker::new(core::ptr::null(), &NOOP_VTABLE);
    unsafe { Waker::from_raw(raw) }
}

impl HiccRuntime for SpinRuntime {
    fn block_on(&self, mut f: Pin<&mut dyn Future<Output = ()>>) {
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);
        while f.as_mut().poll(&mut cx).is_pending() {}
    }
    fn spawn(&self, mut f: Pin<&mut dyn Future<Output = ()>>) {
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);
        while f.as_mut().poll(&mut cx).is_pending() {}
    }
}

hicc_rs::foreign!();

fn new_runtime() -> Box<dyn HiccRuntime> {
    Box::new(SpinRuntime)
}

async fn async_add(a: i32, b: i32) -> i32 {
    let result = a + b;
    std::eprintln!("[async_add] a={}, b={}, result={}", a, b, result);
    result
}

async fn async_hello() -> String {
    String::from("hello async")
}

#[derive(Debug)]
pub struct AsyncCounter {
    pub base: i32,
}

impl AsyncCounter {
    pub fn new(base: i32) -> Self {
        Self { base }
    }
    pub async fn async_increment(&self, delta: i32) -> i32 {
        self.base + delta
    }
    pub async fn async_greet(&self) -> String {
        format!("hello from {}", self.base)
    }
}

fn new_counter(base: i32) -> AsyncCounter {
    AsyncCounter::new(base)
}

#[export_class]
impl AsyncCounter {
    async fn async_increment(&self, delta: i32) -> i32;
    async fn async_greet(&self) -> String;
}

// ---- Generic Self test: async method with body using Self keyword ----
#[derive(Debug)]
pub struct GenericCounter<T: ValueType> {
    pub inner: T,
}

impl<T: ValueType> GenericCounter<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[export_class]
impl<T: ValueType> GenericCounter<T> {
    async fn async_size_check(&self) -> usize {
        core::mem::size_of::<Self>()
    }
}

// ---- Generic Self test: non-async method with body using Self keyword ----
pub struct SimpleWrapper<T: ValueType>(pub T);

impl<T: ValueType> SimpleWrapper<T> {
    pub fn new(val: T) -> Self {
        Self(val)
    }
    pub fn get_size(&self) -> usize {
        core::mem::size_of::<Self>()
    }
}

#[export_class]
impl<T: ValueType> SimpleWrapper<T> {
    fn get_size(&self) -> usize {
        core::mem::size_of::<Self>()
    }
}

#[export_lib(foreign, name = "async_func")]
mod lib {
    use super::*;
    fn new_runtime() -> Box<dyn HiccRuntime>;
    async fn async_add(a: i32, b: i32) -> i32;
    async fn async_hello() -> String;
    fn new_counter(base: i32) -> AsyncCounter;
}

// ---- Tests ----
#[cfg(test)]
mod tests {
    use super::*;
    use hicc_rs::*;

    #[test]
    fn test_generic_self_async() {
        unsafe {
            let counter = AsyncCounter::new(42);
            let gc = GenericCounter { inner: counter };
            let abi: AbiClass<GenericCounter<AsyncCounter>> = transmute(hicc_rs::to_abi(gc));
            let future: AbiClass<Box<dyn Future<Output = usize>>> = transmute(
                (abi.methods.methods.async_size_check)(transmute(&abi)),
            );
            let rt = SpinRuntime;
            let rt_box: Box<dyn HiccRuntime> = Box::new(rt);
            let rt_abi: AbiClass<Box<dyn HiccRuntime>> = transmute(hicc_rs::to_abi(rt_box));
            let size: usize = transmute((future.methods.methods.wait)(
                transmute(future),
                transmute(&rt_abi),
            ));
            assert_eq!(size, core::mem::size_of::<GenericCounter<AsyncCounter>>());
        }
    }

    #[test]
    fn test_generic_self_sync() {
        unsafe {
            let sw = SimpleWrapper(42i32);
            let abi: AbiClass<SimpleWrapper<i32>> = transmute(hicc_rs::to_abi(sw));
            let size: usize = transmute((abi.methods.methods.get_size)(transmute(&abi)));
            assert_eq!(size, core::mem::size_of::<SimpleWrapper<i32>>());
        }
    }
}
