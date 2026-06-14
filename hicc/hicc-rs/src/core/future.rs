use crate::{export_class, AbiType};
use alloc::boxed::Box;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

#[cfg(feature = "cbindgen")]
use crate::{ExportType, TypeRegistry};

struct WaitFuture<F: Future<Output = R>, R>(F, Option<R>);

impl<F: Future<Output = R>, R> WaitFuture<F, R> {
    #[allow(dead_code)]
    pub fn new(f: F) -> Self {
        Self(f, None)
    }
}

impl<F: Future<Output = R>, R> Unpin for WaitFuture<F, R> {}

impl<F: Future<Output = R>, R> Future for WaitFuture<F, R> {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let f = unsafe { Pin::new_unchecked(&mut self.0) };
        match Future::poll(f, ctx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(r) => {
                self.1 = Some(r);
                Poll::Ready(())
            }
        }
    }
}

struct NotifyFuture<F: Future<Output = R>, R>(F, Notify<R>);

impl<F: Future<Output = R>, R> NotifyFuture<F, R> {
    #[allow(dead_code)]
    pub fn new(f: F, notify: Notify<R>) -> Self {
        Self(f, notify)
    }
}

impl<F: Future<Output = R>, R> Unpin for NotifyFuture<F, R> {}

impl<F: Future<Output = R>, R> Future for NotifyFuture<F, R> {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let f = unsafe { Pin::new_unchecked(&mut self.0) };
        match Future::poll(f, ctx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(r) => {
                (self.1.on_return)(
                    crate::to_abi(r),
                    self.1.ctx,
                );
                Poll::Ready(())
            }
        }
    }
}

pub trait HiccRuntime {
    fn block_on(&self, f: Pin<&mut dyn Future<Output = ()>>);
    fn spawn(&self, f: Pin<&mut dyn Future<Output = ()>>);
}

#[repr(C)]
pub struct Notify<R> {
    pub on_return: extern "C" fn(<R as AbiType>::OutputType, *const ()),
    pub ctx: *const (),
}

#[export_class(in_hicc)]
impl Box<dyn HiccRuntime> {}

#[export_class(in_hicc)]
impl<R> Box<dyn Future<Output = R>>
where
    R: Send + 'static,
{
    fn wait(self, r: &Box<dyn HiccRuntime>) -> R {
        let mut b = WaitFuture(unsafe { Pin::new_unchecked(self) }, None);
        r.block_on(unsafe { Pin::new_unchecked(&mut b) });
        b.1.take().unwrap()
    }
    fn async_wait(self, r: &Box<dyn HiccRuntime>, notify: Notify<R>) {
        let mut b = NotifyFuture(unsafe { Pin::new_unchecked(self) }, notify);
        r.spawn(unsafe { Pin::new_unchecked(&mut b) });
    }
}

#[cfg(feature = "cbindgen")]
impl<R> ExportType for Box<dyn Future<Output = R>>
where
    R: Send + 'static,
    R: AbiType,
    <R as AbiType>::OutputType: ExportType,
{
    fn export_name(registry: &mut TypeRegistry) -> alloc::string::String {
        <R as AbiType>::OutputType::export_name(registry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AbiClass;

    #[derive(Debug)]
    pub struct AsyncCounter {
        pub base: i32,
    }

    impl AsyncCounter {
        pub fn new(base: i32) -> Self {
            Self { base }
        }
        async fn async_increment(&self, delta: i32) -> i32 {
            self.base + delta
        }
        async fn async_greet(&self) -> alloc::string::String {
            alloc::format!("hello from {}", self.base)
        }
    }

    #[export_class(in_hicc)]
    impl AsyncCounter {
        async fn async_increment(&self, delta: i32) -> i32;
        async fn async_greet(&self) -> alloc::string::String;
    }

    #[derive(Debug)]
    pub struct GenericCounter<T: crate::ValueType> {
        pub inner: T,
    }

    impl<T: crate::ValueType> GenericCounter<T> {
        async fn async_size_check(&self) -> usize {
            core::mem::size_of::<Self>()
        }
    }

    #[export_class(in_hicc)]
    impl<T: crate::ValueType> GenericCounter<T> {
        async fn async_size_check(&self) -> usize {
            core::mem::size_of::<Self>()
        }
    }

    #[test]
    fn test_hicc_future_poll_ready() {
        let f = async { 42 };
        let mut b = WaitFuture::new(f);
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);
        let pinned = Pin::new(&mut b);
        assert_eq!(pinned.poll(&mut cx), Poll::Ready(()));
        assert_eq!(b.1, Some(42));
    }

    #[test]
    fn test_hicc_future_poll_pending() {
        use core::future::poll_fn;
        let f = poll_fn(|_| Poll::<i32>::Pending);
        let mut b = WaitFuture::new(f);
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);
        let pinned = Pin::new(&mut b);
        match pinned.poll(&mut cx) {
            Poll::Pending => {}
            _ => panic!("expected pending"),
        }
    }

    fn noop_waker() -> core::task::Waker {
        let raw = core::task::RawWaker::new(core::ptr::null(), &NOOP_VTABLE);
        unsafe { core::task::Waker::from_raw(raw) }
    }

    const NOOP_VTABLE: core::task::RawWakerVTable = core::task::RawWakerVTable::new(
        |p| core::task::RawWaker::new(p, &NOOP_VTABLE),
        |_| {},
        |_| {},
        |_| {},
    );

    struct PollLoop;

    impl HiccRuntime for PollLoop {
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

    extern "C" fn on_return_i32(result: <i32 as crate::AbiType>::OutputType, ctx: *const ()) {
        let ptr = ctx as *mut Option<i32>;
        unsafe {
            let rust_val: i32 = crate::from_abi_val::<i32>(result);
            *ptr = Some(rust_val);
        }
    }

    extern "C" fn on_return_string(result: <alloc::string::String as crate::AbiType>::OutputType, ctx: *const ()) {
        let ptr = ctx as *mut Option<alloc::string::String>;
        unsafe {
            let rust_val: alloc::string::String = crate::from_abi_val::<alloc::string::String>(result);
            *ptr = Some(rust_val);
        }
    }

    #[test]
    fn test_async_wait_i32() {
        type Rt = Box<dyn HiccRuntime>;
        type FutI32 = Box<dyn Future<Output = i32>>;
        unsafe {
            let rt: AbiClass<Rt> =
                crate::cabi::transmute(crate::to_abi::<Rt>(Box::new(PollLoop)));
            let f: FutI32 = Box::new(async { 42 });
            let future: AbiClass<FutI32> =
                crate::cabi::transmute(crate::to_abi::<FutI32>(f));
            let mut result_slot: Option<i32> = None;
            let notify = Notify::<i32> {
                on_return: on_return_i32 as extern "C" fn(<i32 as crate::AbiType>::OutputType, *const ()),
                ctx: &mut result_slot as *mut Option<i32> as *const (),
            };
            (future.methods.methods.async_wait)(
                crate::cabi::transmute(future),
                crate::cabi::transmute(&rt),
                crate::cabi::transmute(crate::to_abi::<Notify<i32>>(notify)),
            );
            assert_eq!(result_slot, Some(42));
        }
    }

    #[test]
    fn test_async_wait_string() {
        type Rt = Box<dyn HiccRuntime>;
        type FutStr = Box<dyn Future<Output = alloc::string::String>>;
        type Str = alloc::string::String;
        unsafe {
            let rt: AbiClass<Rt> =
                crate::cabi::transmute(crate::to_abi::<Rt>(Box::new(PollLoop)));
            let f: FutStr = Box::new(async { Str::from("hello") });
            let future: AbiClass<FutStr> =
                crate::cabi::transmute(crate::to_abi::<FutStr>(f));
            let mut result_slot: Option<Str> = None;
            let notify = Notify::<Str> {
                on_return: on_return_string as extern "C" fn(<Str as crate::AbiType>::OutputType, *const ()),
                ctx: &mut result_slot as *mut Option<Str> as *const (),
            };
            (future.methods.methods.async_wait)(
                crate::cabi::transmute(future),
                crate::cabi::transmute(&rt),
                crate::cabi::transmute(crate::to_abi::<Notify<Str>>(notify)),
            );
            assert_eq!(result_slot, Some(Str::from("hello")));
        }
    }

    #[test]
    fn test_wait_i32() {
        type Rt = Box<dyn HiccRuntime>;
        type FutI32 = Box<dyn Future<Output = i32>>;
        unsafe {
            let rt: AbiClass<Rt> =
                crate::cabi::transmute(crate::to_abi::<Rt>(Box::new(PollLoop)));
            let f: FutI32 = Box::new(async { 42 });
            let future: AbiClass<FutI32> =
                crate::cabi::transmute(crate::to_abi::<FutI32>(f));
            let result: i32 = crate::cabi::transmute((future.methods.methods.wait)(
                crate::cabi::transmute(future),
                crate::cabi::transmute(&rt),
            ));
            assert_eq!(result, 42);
        }
    }

    #[test]
    fn test_wait_string() {
        type Rt = Box<dyn HiccRuntime>;
        type FutStr = Box<dyn Future<Output = alloc::string::String>>;
        type Str = alloc::string::String;
        unsafe {
            let rt: AbiClass<Rt> =
                crate::cabi::transmute(crate::to_abi::<Rt>(Box::new(PollLoop)));
            let f: FutStr = Box::new(async { Str::from("hello") });
            let future: AbiClass<FutStr> =
                crate::cabi::transmute(crate::to_abi::<FutStr>(f));
            let result: AbiClass<Str> = crate::cabi::transmute((future.methods.methods.wait)(
                crate::cabi::transmute(future),
                crate::cabi::transmute(&rt),
            ));
            let rust_str: Str = *result.take_boxed();
            assert_eq!(rust_str, Str::from("hello"));
        }
    }

    #[test]
    fn test_async_associated_method_wait_i32() {
        type Rt = Box<dyn HiccRuntime>;
        type FutI32 = Box<dyn Future<Output = i32>>;
        type Counter = AsyncCounter;
        unsafe {
            let rt: AbiClass<Rt> =
                crate::cabi::transmute(crate::to_abi::<Rt>(Box::new(PollLoop)));

            let counter = AsyncCounter { base: 100 };
            let counter_abi: AbiClass<Counter> =
                crate::cabi::transmute(crate::to_abi::<Counter>(counter));

            let future: AbiClass<FutI32> = crate::cabi::transmute(
                (counter_abi.methods.methods.async_increment)(
                    crate::cabi::transmute(&counter_abi),
                    crate::cabi::transmute(crate::to_abi::<i32>(5)),
                ),
            );
            let result: i32 = crate::cabi::transmute((future.methods.methods.wait)(
                crate::cabi::transmute(future),
                crate::cabi::transmute(&rt),
            ));
            assert_eq!(result, 105);
        }
    }

    #[test]
    fn test_async_associated_method_async_wait_string() {
        type Rt = Box<dyn HiccRuntime>;
        type FutStr = Box<dyn Future<Output = alloc::string::String>>;
        type Str = alloc::string::String;
        type Counter = AsyncCounter;
        unsafe {
            let rt: AbiClass<Rt> =
                crate::cabi::transmute(crate::to_abi::<Rt>(Box::new(PollLoop)));

            let counter = AsyncCounter { base: 42 };
            let counter_abi: AbiClass<Counter> =
                crate::cabi::transmute(crate::to_abi::<Counter>(counter));

            let mut result_slot: Option<Str> = None;
            let future: AbiClass<FutStr> = crate::cabi::transmute(
                (counter_abi.methods.methods.async_greet)(
                    crate::cabi::transmute(&counter_abi),
                ),
            );
            let notify = Notify::<Str> {
                on_return: on_return_string as extern "C" fn(<Str as crate::AbiType>::OutputType, *const ()),
                ctx: &mut result_slot as *mut Option<Str> as *const (),
            };
            (future.methods.methods.async_wait)(
                crate::cabi::transmute(future),
                crate::cabi::transmute(&rt),
                crate::cabi::transmute(crate::to_abi::<Notify<Str>>(notify)),
            );
            assert_eq!(result_slot, Some(Str::from("hello from 42")));
        }
    }

    #[test]
    fn test_generic_self_async_method() {
        type Rt = Box<dyn HiccRuntime>;
        type FutUsize = Box<dyn Future<Output = usize>>;
        type Counter = GenericCounter<i32>;
        unsafe {
            let rt: AbiClass<Rt> =
                crate::cabi::transmute(crate::to_abi::<Rt>(Box::new(PollLoop)));
            let counter = GenericCounter { inner: 42i32 };
            let counter_abi: AbiClass<Counter> =
                crate::cabi::transmute(crate::to_abi::<Counter>(counter));
            let future: AbiClass<FutUsize> = crate::cabi::transmute(
                (counter_abi.methods.methods.async_size_check)(
                    crate::cabi::transmute(&counter_abi),
                ),
            );
            let result: usize = crate::cabi::transmute((future.methods.methods.wait)(
                crate::cabi::transmute(future),
                crate::cabi::transmute(&rt),
            ));
            assert_eq!(result, core::mem::size_of::<GenericCounter<i32>>());
        }
    }
}
