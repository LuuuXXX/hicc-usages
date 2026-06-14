#![feature(specialization)]
use hicc_rs::export_class;
use hicc_rs::*;

pub struct Point(i32, i32);

impl Point {
    fn x(&self) -> i32 {
        self.0
    }
    fn y(&self) -> i32 {
        self.1
    }
}

pub struct Counter(i32);

impl Counter {
    fn inc(&mut self) {
        self.0 += 1;
    }
    fn val(&self) -> i32 {
        self.0
    }
}

// Test: export_class mod block exporting multiple types
// Each impl block inside the mod is independently processed
#[export_class]
mod classes {
    use super::*;
    impl Point {
        fn x(&self) -> i32;
        fn y(&self) -> i32;
    }
    impl Counter {
        fn inc(&mut self);
        fn val(&self) -> i32;
    }
}

#[test]
fn test_mod_class_batch_export() {
    unsafe {
        let p: AbiClass<Point> = transmute(crate::to_abi(Point(3, 5)));
        let x: i32 = transmute((p.methods.methods.x)(transmute(&p)));
        assert_eq!(x, 3);
        let y: i32 = transmute((p.methods.methods.y)(transmute(&p)));
        assert_eq!(y, 5);

        let mut c: AbiClass<Counter> =
            transmute(crate::to_abi(Counter(0)));
        (c.methods.methods.inc)(transmute(&mut c));
        (c.methods.methods.inc)(transmute(&mut c));
        let val: i32 = transmute((c.methods.methods.val)(transmute(&c)));
        assert_eq!(val, 2);
    }
}
