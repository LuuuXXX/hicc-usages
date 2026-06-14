#![feature(specialization)]
hicc_rs::foreign!();
use hicc_rs::export_class;
use hicc_rs::*;

type RefSliceU8 = &'static [u8];

#[export_class(foreign)]
impl &[u8] {
    fn len(&self) -> usize;
}

#[test]
fn test_ref_slice_foreign() {
    unsafe {
        static DATA: [u8; 5] = [10, 20, 30, 40, 50];
        let slice: &'static [u8] = &DATA;
        let abi: AbiClass<crate::hicc::Foreign<RefSliceU8>> = transmute(crate::to_abi(slice));
        let len: usize = transmute((abi.methods.methods.len)(transmute(&abi)));
        assert_eq!(len, 5);
    }
}
