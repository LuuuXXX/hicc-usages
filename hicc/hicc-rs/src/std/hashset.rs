use crate::export_class;
use std::collections::hash_set::{IntoIter, Iter};
use std::collections::HashSet;

#[export_class(in_hicc)]
impl<T> Iter<'_, T> {
    fn next(&mut self) -> Option<&T>;
}

#[export_class(in_hicc)]
impl<T> IntoIter<T> {
    fn next(&mut self) -> Option<T>;
}

#[export_class(in_hicc)]
impl<T: std::hash::Hash + std::cmp::Eq> HashSet<T> {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn contains(&self, val: &T) -> bool;
    fn insert(&mut self, val: T) -> bool;
    fn remove(&mut self, val: &T) -> bool;
    fn iter(&self) -> Iter<'_, T>;
    fn into_iter(self) -> IntoIter<T>;
}

#[cfg(test)]
mod test {
    use crate::*;
    use std::collections::hash_set::Iter;
    use std::collections::HashSet;

    #[test]
    fn test_hashset_basic() {
        unsafe {
            let mut abi_set: AbiClass<HashSet<i32>> =
                transmute(crate::to_abi::<HashSet<i32>>(HashSet::new()));
            let len: usize = transmute((abi_set.methods.methods.len)(transmute(&abi_set)));
            assert_eq!(len, 0);

            let inserted: bool = transmute((abi_set.methods.methods.insert)(
                transmute(&mut abi_set),
                transmute(crate::to_abi(42)),
            ));
            assert_eq!(inserted, true);

            let len: usize = transmute((abi_set.methods.methods.len)(transmute(&abi_set)));
            assert_eq!(len, 1);
        }
    }

    #[test]
    fn test_hashset_iter() {
        unsafe {
            let mut abi_set: AbiClass<HashSet<i32>> =
                transmute(crate::to_abi::<HashSet<i32>>(HashSet::new()));

            (abi_set.methods.methods.insert)(transmute(&mut abi_set), transmute(crate::to_abi(10)));
            (abi_set.methods.methods.insert)(transmute(&mut abi_set), transmute(crate::to_abi(20)));

            let mut abi_iter: AbiClass<Iter<'static, i32>> =
                transmute((abi_set.methods.methods.iter)(transmute(&abi_set)));

            // First: should be Some
            let abi_opt1: AbiClass<Option<&'static i32>> =
                transmute((abi_iter.methods.methods.next)(transmute(&mut abi_iter)));
            let is_none1: bool =
                transmute((abi_opt1.methods.methods.is_none)(transmute(&abi_opt1)));
            assert_eq!(is_none1, false);

            // Second: should be Some
            let abi_opt2: AbiClass<Option<&'static i32>> =
                transmute((abi_iter.methods.methods.next)(transmute(&mut abi_iter)));
            let is_none2: bool =
                transmute((abi_opt2.methods.methods.is_none)(transmute(&abi_opt2)));
            assert_eq!(is_none2, false);

            // Third: should be None
            let abi_opt3: AbiClass<Option<&'static i32>> =
                transmute((abi_iter.methods.methods.next)(transmute(&mut abi_iter)));
            let is_none3: bool =
                transmute((abi_opt3.methods.methods.is_none)(transmute(&abi_opt3)));
            assert_eq!(is_none3, true);
        }
    }
}
