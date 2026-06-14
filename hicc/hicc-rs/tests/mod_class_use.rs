#![feature(specialization)]
use hicc_rs::export_class;
use hicc_rs::*;

pub struct Pair<T, V>(T, V);

impl<T, V> Pair<T, V> {
    fn first(&self) -> &T {
        &self.0
    }
    fn second(&self) -> &V {
        &self.1
    }
}

// Test: export_class mod block with `use super::*;` inside
// This was the bug where replace_semicolons incorrectly replaced the `;`
// in `use super::*;` with `{}`. The new custom parser handles this correctly
// because `use super::*;` is parsed as a syn::Item (ItemUse), not a fn declaration.
#[export_class]
mod classes {
    use super::*;
    impl<T, V> Pair<T, V> {
        fn first(&self) -> &T;
        fn second(&self) -> &V;
    }
}

#[test]
fn test_mod_class_with_use() {
    unsafe {
        let p: AbiClass<Pair<i32, i64>> = transmute(
            crate::to_abi(Pair(10, 20)),
        );
        let first: &i32 = transmute((p.methods.methods.first)(transmute(&p)));
        assert_eq!(*first, 10);
        let second: &i64 = transmute((p.methods.methods.second)(transmute(&p)));
        assert_eq!(*second, 20);
    }
}
