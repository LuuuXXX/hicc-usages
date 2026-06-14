use crate::{export_class, IsClass, ValueType};

type Array<T, const N: usize> = [T; N];

#[export_class(in_hicc)]
impl<T: ValueType<Type = IsClass>, const N: usize> Array<T, N> {
    fn len(&self) -> usize {
        N
    }
    fn set(&mut self, idx: usize, val: T) {
        self[idx] = val;
    }
    fn get(&self, idx: usize) -> &T {
        &self[idx]
    }
    fn get_mut(&mut self, idx: usize) -> &mut T {
        &mut self[idx]
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_array_i32() {
        type Item = [i32; 3];
        unsafe {
            let abi_array: [i32; 3] = transmute(<Item as AbiType>::into_abi(transmute([1, 2, 3])));
            assert_eq!(abi_array[0], 1);
            assert_eq!(abi_array[1], 2);
            assert_eq!(abi_array[2], 3);
        }
    }

    #[test]
    fn test_array_opt_i32() {
        type Item = [Option<i32>; 3];
        unsafe {
            let mut abi_array: AbiClass<Item> =
                transmute(<Item as AbiType>::into_abi(transmute([
                    None,
                    None,
                    Some(3),
                ])));
            assert!(abi_array.is_value());
            assert!(abi_array.is_mut());
            assert!(!abi_array.this.is_null());

            let len: usize = transmute((abi_array.methods.methods.len)(transmute(&abi_array)));
            assert_eq!(len, 3);

            (abi_array.methods.methods.set)(
                transmute(&mut abi_array),
                transmute(0_usize),
                transmute(crate::to_abi::<Option<i32>>(Some(100))),
            );
            let abi_item: AbiClass<Option<i32>> = transmute((abi_array.methods.methods.get)(
                transmute(&abi_array),
                transmute(0_usize),
            ));
            assert!(abi_item.is_pointer());
            assert!(abi_item.is_const());

            let val: &i32 = transmute((abi_item.methods.methods.as_ref)(transmute(&abi_item)));
            assert_eq!(val, &100);
        }
    }
}
