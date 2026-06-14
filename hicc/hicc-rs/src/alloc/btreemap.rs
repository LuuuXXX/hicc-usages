use crate::export_class;
use alloc::collections::btree_map::{IntoIter, Iter, IterMut};
use alloc::collections::BTreeMap;

#[export_class(in_hicc)]
impl<K, V> Iter<'_, K, V> {
    fn next(&mut self) -> Option<(&K, &V)>;
}

#[export_class(in_hicc)]
impl<K, V> IterMut<'_, K, V> {
    fn next(&mut self) -> Option<(&K, &mut V)>;
}

#[export_class(in_hicc)]
impl<K, V> IntoIter<K, V> {
    fn next(&mut self) -> Option<(K, V)>;
}

#[export_class(in_hicc)]
impl<K: ::core::cmp::Ord, V> BTreeMap<K, V> {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn contains_key(&self, key: &K) -> bool;
    fn get(&self, key: &K) -> Option<&V>;
    fn insert(&mut self, key: K, val: V) -> Option<V>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn iter(&self) -> Iter<'_, K, V>;
    fn iter_mut(&mut self) -> IterMut<'_, K, V>;
    fn into_iter(self) -> IntoIter<K, V>;
}

#[cfg(test)]
mod test {
    use crate::*;
    use alloc::collections::btree_map::Iter;
    use alloc::collections::BTreeMap;

    #[test]
    fn test_btreemap_basic() {
        unsafe {
            let mut abi_map: AbiClass<BTreeMap<i32, i32>> =
                transmute(crate::to_abi::<BTreeMap<i32, i32>>(BTreeMap::new()));
            let len: usize = transmute((abi_map.methods.methods.len)(transmute(&abi_map)));
            assert_eq!(len, 0);

            (abi_map.methods.methods.insert)(
                transmute(&mut abi_map),
                transmute(crate::to_abi(1)),
                transmute(crate::to_abi(100)),
            );
            let len: usize = transmute((abi_map.methods.methods.len)(transmute(&abi_map)));
            assert_eq!(len, 1);
        }
    }

    #[test]
    fn test_btreemap_iter() {
        unsafe {
            let mut abi_map: AbiClass<BTreeMap<i32, i32>> =
                transmute(crate::to_abi::<BTreeMap<i32, i32>>(BTreeMap::new()));

            (abi_map.methods.methods.insert)(
                transmute(&mut abi_map),
                transmute(crate::to_abi(1)),
                transmute(crate::to_abi(10)),
            );
            (abi_map.methods.methods.insert)(
                transmute(&mut abi_map),
                transmute(crate::to_abi(2)),
                transmute(crate::to_abi(20)),
            );

            let mut abi_iter: AbiClass<Iter<'static, i32, i32>> =
                transmute((abi_map.methods.methods.iter)(transmute(&abi_map)));

            // First entry: should be Some
            let abi_entry_opt: AbiClass<Option<(&'static i32, &'static i32)>> =
                transmute((abi_iter.methods.methods.next)(transmute(&mut abi_iter)));
            let is_none: bool = transmute((abi_entry_opt.methods.methods.is_none)(transmute(
                &abi_entry_opt,
            )));
            assert_eq!(is_none, false);

            // Second entry: should be Some
            let abi_entry_opt2: AbiClass<Option<(&'static i32, &'static i32)>> =
                transmute((abi_iter.methods.methods.next)(transmute(&mut abi_iter)));
            let is_none2: bool = transmute((abi_entry_opt2.methods.methods.is_none)(transmute(
                &abi_entry_opt2,
            )));
            assert_eq!(is_none2, false);

            // Third entry: should be None
            let abi_entry_opt3: AbiClass<Option<(&'static i32, &'static i32)>> =
                transmute((abi_iter.methods.methods.next)(transmute(&mut abi_iter)));
            let is_none3: bool = transmute((abi_entry_opt3.methods.methods.is_none)(transmute(
                &abi_entry_opt3,
            )));
            assert_eq!(is_none3, true);
        }
    }
}
