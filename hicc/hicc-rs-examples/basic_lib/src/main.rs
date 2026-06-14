#![feature(specialization)]

use example_basic_lib::{lib, Container, Point};
use hicc_rs::transmute;
#[cfg(feature = "cbindgen")]
use hicc_rs::TypeRegistry;

fn main() {
    unsafe {
        let lib = lib::demo();

        // Plain i32
        let sum: i32 = transmute((lib.add)(transmute(3i32), transmute(4i32)));
        println!("add(3, 4) = {}", sum);
        let neg: i32 = transmute((lib.negate)(transmute(5i32)));
        println!("negate(5) = {}", neg);

        // Container<i32>
        let container_abi = transmute(hicc_rs::to_abi(Container(42)));
        let val: i32 = transmute((lib.container_value)(container_abi));
        println!("container_value(42) = {}", val);

        // Option<i32>
        let opt_abi = transmute(hicc_rs::to_abi(Some(99)));
        let doubled: i64 = transmute((lib.double_option)(opt_abi));
        println!("double_option(Some(99)) = {}", doubled);

        // Str (&'static str)
        let str_abi = transmute(hicc_rs::to_abi("hello"));
        let len: usize = transmute((lib.check_str)(str_abi));
        println!("check_str(\"hello\") = {}", len);

        // Slice<Option<i32>>
        let items: &'static [Option<i32>] = &[Some(10), None, Some(30)];
        let slice_abi = transmute(hicc_rs::to_abi(items));
        let count: usize = transmute((lib.count_some)(slice_abi));
        println!("count_some(&[Some(10), None, Some(30)]) = {}", count);

        // Array<&'static str, 3>
        let arr: [&'static str; 3] = ["a", "bb", "ccc"];
        let arr_abi = transmute(hicc_rs::to_abi(arr));
        let total: usize = transmute((lib.total_len)(arr_abi));
        println!("total_len([\"a\", \"bb\", \"ccc\"]) = {}", total);

        // Plain #[repr(C)] struct (no export_class)
        let p1 = Point { x: 10, y: 20 };
        let p2 = Point { x: 1, y: 2 };
        let sum: Point = transmute((lib.add_point)(transmute(p1), transmute(p2)));
        println!("add_point((10,20), (1,2)) = ({}, {})", sum.x, sum.y);
    }

    #[cfg(feature = "cbindgen")]
    {
        let mut registry = TypeRegistry::new();
        let entry = example_basic_lib::lib::demo_cbindgen(&mut registry);
        let code = registry.to_cbindgen_code(entry);
        println!("\n=== Cbindgen generated code ===");
        println!("{}", code);
        println!("=== End ===");
    }

    println!("Basic lib example passed!");
}
