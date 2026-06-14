use crate::export_class;
use std::sync::{Mutex, MutexGuard, TryLockError};

#[export_class(in_hicc)]
impl<T> Mutex<T> {
    fn lock(&self) -> MutexGuard<'_, T> {
        self.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        match self.try_lock() {
            Ok(guard) => Some(guard),
            Err(TryLockError::Poisoned(p)) => Some(p.into_inner()),
            Err(TryLockError::WouldBlock) => None,
        }
    }

    fn into_inner(self) -> T {
        self.into_inner().unwrap_or_else(|e| e.into_inner())
    }

    fn get_mut(&mut self) -> &mut T {
        self.get_mut().unwrap_or_else(|e| e.into_inner())
    }

    fn is_poisoned(&self) -> bool {
        self.is_poisoned()
    }
}

#[export_class(in_hicc)]
impl<T> MutexGuard<'_, T> {
    fn get(&self) -> &T {
        &**self
    }

    fn get_mut(&mut self) -> &mut T {
        &mut **self
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use std::sync::{Mutex, MutexGuard};

    #[test]
    fn test_mutex_lock_get() {
        unsafe {
            let abi_mutex: AbiClass<Mutex<i32>> = transmute(
                crate::to_abi(Mutex::new(42)),
            );
            let abi_guard: AbiClass<MutexGuard<'static, i32>> =
                transmute((abi_mutex.methods.methods.lock)(transmute(&abi_mutex)));
            let val: &i32 = transmute((abi_guard.methods.methods.get)(transmute(&abi_guard)));
            assert_eq!(*val, 42);
        }
    }

    #[test]
    fn test_mutex_lock_get_mut() {
        unsafe {
            let abi_mutex: AbiClass<Mutex<i32>> = transmute(
                crate::to_abi(Mutex::new(42)),
            );
            let abi_guard: AbiClass<MutexGuard<'static, i32>> =
                transmute((abi_mutex.methods.methods.lock)(transmute(&abi_mutex)));
            let val: &mut i32 =
                transmute((abi_guard.methods.methods.get_mut)(transmute(&abi_guard)));
            assert_eq!(*val, 42);
            *val = 99;
        }
    }

    #[test]
    fn test_mutex_into_inner() {
        unsafe {
            let abi_mutex: AbiClass<Mutex<i32>> = transmute(
                crate::to_abi(Mutex::new(42)),
            );
            let inner: i32 =
                transmute((abi_mutex.methods.methods.into_inner)(transmute(abi_mutex)));
            assert_eq!(inner, 42);
        }
    }

    #[test]
    fn test_mutex_get_mut() {
        unsafe {
            let mut abi_mutex: AbiClass<Mutex<i32>> =
                transmute(crate::to_abi(Mutex::new(
                    42,
                )));
            let val: &mut i32 = transmute((abi_mutex.methods.methods.get_mut)(transmute(
                &mut abi_mutex,
            )));
            *val = 99;
            let inner: i32 =
                transmute((abi_mutex.methods.methods.into_inner)(transmute(abi_mutex)));
            assert_eq!(inner, 99);
        }
    }

    #[test]
    fn test_mutex_try_lock() {
        unsafe {
            let abi_mutex: AbiClass<Mutex<i32>> = transmute(
                crate::to_abi(Mutex::new(42)),
            );
            let abi_opt: AbiClass<Option<MutexGuard<'static, i32>>> =
                transmute((abi_mutex.methods.methods.try_lock)(transmute(&abi_mutex)));
            let is_none: bool = transmute((abi_opt.methods.methods.is_none)(transmute(&abi_opt)));
            assert!(!is_none);
            let abi_guard: AbiClass<MutexGuard<'static, i32>> =
                transmute((abi_opt.methods.methods.unwrap)(transmute(abi_opt)));
            let val: &mut i32 =
                transmute((abi_guard.methods.methods.get_mut)(transmute(&abi_guard)));
            assert_eq!(*val, 42);
        }
    }

    #[test]
    fn test_mutex_is_poisoned() {
        unsafe {
            let abi_mutex: AbiClass<Mutex<i32>> = transmute(
                crate::to_abi(Mutex::new(42)),
            );
            let poisoned: bool = transmute((abi_mutex.methods.methods.is_poisoned)(transmute(
                &abi_mutex,
            )));
            assert!(!poisoned);
        }
    }
}
