use crate::export_class;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[export_class(in_hicc)]
impl<T> RwLock<T> {
    fn read(&self) -> RwLockReadGuard<'_, T> {
        self.read().unwrap_or_else(|e| e.into_inner())
    }

    fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.write().unwrap_or_else(|e| e.into_inner())
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
impl<T> RwLockReadGuard<'_, T> {
    fn get(&self) -> &T {
        &**self
    }
}

#[export_class(in_hicc)]
impl<T> RwLockWriteGuard<'_, T> {
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
    use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

    #[test]
    fn test_rwlock_read() {
        unsafe {
            let abi_lock: AbiClass<RwLock<i32>> =
                transmute(crate::to_abi(RwLock::new(42)));
            let abi_guard: AbiClass<RwLockReadGuard<'static, i32>> =
                transmute((abi_lock.methods.methods.read)(transmute(&abi_lock)));
            let val: &i32 = transmute((abi_guard.methods.methods.get)(transmute(&abi_guard)));
            assert_eq!(*val, 42);
        }
    }

    #[test]
    fn test_rwlock_write() {
        unsafe {
            let abi_lock: AbiClass<RwLock<i32>> =
                transmute(crate::to_abi(RwLock::new(42)));
            let abi_guard: AbiClass<RwLockWriteGuard<'static, i32>> =
                transmute((abi_lock.methods.methods.write)(transmute(&abi_lock)));
            let val: &mut i32 =
                transmute((abi_guard.methods.methods.get_mut)(transmute(&abi_guard)));
            assert_eq!(*val, 42);
            *val = 99;
        }
    }

    #[test]
    fn test_rwlock_into_inner() {
        unsafe {
            let abi_lock: AbiClass<RwLock<i32>> =
                transmute(crate::to_abi(RwLock::new(42)));
            let inner: i32 = transmute((abi_lock.methods.methods.into_inner)(transmute(abi_lock)));
            assert_eq!(inner, 42);
        }
    }

    #[test]
    fn test_rwlock_get_mut() {
        unsafe {
            let mut abi_lock: AbiClass<RwLock<i32>> =
                transmute(crate::to_abi(RwLock::new(42)));
            let val: &mut i32 =
                transmute((abi_lock.methods.methods.get_mut)(transmute(&mut abi_lock)));
            assert_eq!(*val, 42);
            *val = 100;
            let inner: i32 = transmute((abi_lock.methods.methods.into_inner)(transmute(abi_lock)));
            assert_eq!(inner, 100);
        }
    }

    #[test]
    fn test_rwlock_is_poisoned() {
        unsafe {
            let abi_lock: AbiClass<RwLock<i32>> =
                transmute(crate::to_abi(RwLock::new(42)));
            let poisoned: bool =
                transmute((abi_lock.methods.methods.is_poisoned)(transmute(&abi_lock)));
            assert!(!poisoned);
        }
    }
}
