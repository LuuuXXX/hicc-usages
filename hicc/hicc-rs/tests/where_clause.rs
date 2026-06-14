#![feature(specialization)]
use hicc_rs::export_class;
use hicc_rs::*;

pub struct WhereT(i32);

impl WhereT {
    fn get(&self) -> i32 {
        self.0
    }
}

#[export_class]
impl WhereT {
    fn get(&self) -> i32;
}

pub struct WhereU(bool);

impl WhereU {
    fn get(&self) -> bool {
        self.0
    }
}

#[export_class]
impl WhereU {
    fn get(&self) -> bool;
}

// Where clause bounds like T: ValueType require ValueType<Type = IsClass>.
pub struct WhereStruct<T, U>(T, U);

impl<T, U> WhereStruct<T, U>
where
    T: ValueType,
    U: ValueType,
{
    fn count(&self) -> i32 {
        2
    }
}

#[export_class]
impl<T, U> WhereStruct<T, U>
where
    T: ValueType,
    U: ValueType,
{
    fn count(&self) -> i32;
}

#[test]
fn test_where_clause() {
    unsafe {
        let wt = WhereT(42);
        let wu = WhereU(true);
        let ws = WhereStruct(wt, wu);
        let abi: AbiClass<WhereStruct<WhereT, WhereU>> =
            transmute(crate::to_abi(ws));
        let cnt: i32 = transmute((abi.methods.methods.count)(transmute(&abi)));
        assert_eq!(cnt, 2);
    }
}
