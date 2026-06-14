use crate::export_class;
use core::cell::Cell;
use core::cell::RefCell;

#[export_class(in_hicc)]
impl<T> Cell<T> {
    fn set(&self, val: T);
    fn replace(&self, val: T) -> T;
    fn into_inner(self) -> T;
    fn as_ptr(&self) -> *mut T;
}

#[export_class(in_hicc)]
impl<T> RefCell<T> {
    fn replace(&self, val: T) -> T;
    fn into_inner(self) -> T;
    fn get_mut(&mut self) -> &mut T;
    fn as_ptr(&self) -> *mut T;
}

#[cfg(test)]
mod test {
    use crate::*;
    use core::cell::Cell;
    use core::cell::RefCell;

    #[test]
    fn test_cell_i32() {
        unsafe {
            let abi_cell: AbiClass<Cell<i32>> =
                transmute(crate::to_abi(Cell::new(42)));
            let rpl: i32 = transmute((abi_cell.methods.methods.replace)(
                transmute(&abi_cell),
                transmute(100i32),
            ));
            assert_eq!(rpl, 42);
            let val: i32 = transmute((abi_cell.methods.methods.into_inner)(transmute(abi_cell)));
            assert_eq!(val, 100);
        }
    }

    #[test]
    fn test_cell_set_replace() {
        unsafe {
            let abi_cell: AbiClass<Cell<i32>> =
                transmute(crate::to_abi(Cell::new(10)));
            (abi_cell.methods.methods.set)(transmute(&abi_cell), transmute(20i32));
            let rpl: i32 = transmute((abi_cell.methods.methods.replace)(
                transmute(&abi_cell),
                transmute(30i32),
            ));
            assert_eq!(rpl, 20);
            let val: i32 = transmute((abi_cell.methods.methods.into_inner)(transmute(abi_cell)));
            assert_eq!(val, 30);
        }
    }

    #[test]
    fn test_refcell_i32() {
        unsafe {
            let abi_refcell: AbiClass<RefCell<i32>> =
                transmute(crate::to_abi(RefCell::new(42)));
            let rpl: i32 = transmute((abi_refcell.methods.methods.replace)(
                transmute(&abi_refcell),
                transmute(100i32),
            ));
            assert_eq!(rpl, 42);
            let val: i32 = transmute((abi_refcell.methods.methods.into_inner)(transmute(
                abi_refcell,
            )));
            assert_eq!(val, 100);
        }
    }

    #[test]
    fn test_cell_as_ptr() {
        unsafe {
            let abi_cell: AbiClass<Cell<i32>> =
                transmute(crate::to_abi(Cell::new(42)));
            let ptr: *mut i32 = transmute((abi_cell.methods.methods.as_ptr)(transmute(&abi_cell)));
            assert_eq!(*ptr, 42);
            *ptr = 99;
            let val: i32 = transmute((abi_cell.methods.methods.into_inner)(transmute(abi_cell)));
            assert_eq!(val, 99);
        }
    }

    #[test]
    fn test_refcell_as_ptr() {
        unsafe {
            let abi_refcell: AbiClass<RefCell<i32>> =
                transmute(crate::to_abi(RefCell::new(42)));
            let ptr: *mut i32 = transmute((abi_refcell.methods.methods.as_ptr)(transmute(
                &abi_refcell,
            )));
            assert_eq!(*ptr, 42);
            *ptr = 99;
            let val: i32 = transmute((abi_refcell.methods.methods.into_inner)(transmute(
                abi_refcell,
            )));
            assert_eq!(val, 99);
        }
    }

    #[test]
    fn test_refcell_get_mut() {
        unsafe {
            let mut abi_refcell: AbiClass<RefCell<i32>> =
                transmute(crate::to_abi(RefCell::new(42)));
            let val: &mut i32 = transmute((abi_refcell.methods.methods.get_mut)(transmute(
                &mut abi_refcell,
            )));
            assert_eq!(*val, 42);
            *val = 99;
            let val2: i32 = transmute((abi_refcell.methods.methods.into_inner)(transmute(
                abi_refcell,
            )));
            assert_eq!(val2, 99);
        }
    }
}
