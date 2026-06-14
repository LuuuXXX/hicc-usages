use crate::export_class;
use alloc::vec::Vec;

#[export_class(in_hicc)]
impl<T> Vec<T> {
    fn len(&self) -> usize;
    fn push(&mut self, val: T);
    fn pop(&mut self) -> Option<T>;
    fn get(&self, idx: usize) -> &T {
        &self[idx]
    }
    fn get_mut(&mut self, idx: usize) -> &mut T {
        &mut self[idx]
    }
    fn as_slice(&self) -> &[T];
}

#[cfg(test)]
mod test {
    use crate::*;
    use alloc::vec;
    use alloc::vec::Vec;

    #[test]
    fn test_vec_i32() {
        unsafe {
            let mut abi_vec: AbiClass<Vec<i32>> = transmute(crate::to_abi::<Vec<i32>>(Vec::new()));
            let len: usize = transmute((abi_vec.methods.methods.len)(transmute(&abi_vec)));
            assert_eq!(len, 0);

            (abi_vec.methods.methods.push)(transmute(&mut abi_vec), transmute(crate::to_abi(10)));
            (abi_vec.methods.methods.push)(transmute(&mut abi_vec), transmute(crate::to_abi(20)));
            let len: usize = transmute((abi_vec.methods.methods.len)(transmute(&abi_vec)));
            assert_eq!(len, 2);

            let val: &i32 = transmute((abi_vec.methods.methods.get)(
                transmute(&abi_vec),
                transmute(0_usize),
            ));
            assert_eq!(val, &10);
        }
    }

    #[test]
    fn test_vec_pop() {
        unsafe {
            let mut abi_vec: AbiClass<Vec<i32>> = transmute(crate::to_abi(vec![1, 2, 3]));
            let len: usize = transmute((abi_vec.methods.methods.len)(transmute(&abi_vec)));
            assert_eq!(len, 3);

            let abi_pop: AbiClass<Option<i32>> =
                transmute((abi_vec.methods.methods.pop)(transmute(&mut abi_vec)));
            let val: i32 = transmute((abi_pop.methods.methods.unwrap)(transmute(abi_pop)));
            assert_eq!(val, 3);

            let len: usize = transmute((abi_vec.methods.methods.len)(transmute(&abi_vec)));
            assert_eq!(len, 2);
        }
    }
}
