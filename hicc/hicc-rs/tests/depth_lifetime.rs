#![feature(specialization)]
use hicc_rs::export_class;
use hicc_rs::*;

// Test: export_class with a struct having multiple lifetime parameters.
// The macro strips all lifetimes from generics declarations and replaces
// them with 'static in the self type via make_static_ref.
pub struct DeepRef<'a, 'b, T, U> {
    t: &'a T,
    u: &'b U,
}

impl<'a, 'b, T, U> DeepRef<'a, 'b, T, U> {
    fn get_t(&self) -> &'a T {
        self.t
    }
    fn get_u(&self) -> &'b U {
        self.u
    }
}

#[export_class]
impl<'a, 'b, T, U> DeepRef<'a, 'b, T, U> {
    fn get_t(&self) -> &'a T;
    fn get_u(&self) -> &'b U;
}

#[test]
fn test_depth_lifetime() {
    unsafe {
        static T_VAL: i32 = 10;
        static U_VAL: i64 = 20;
        let dr = DeepRef {
            t: &T_VAL,
            u: &U_VAL,
        };
        let abi: AbiClass<DeepRef<'static, 'static, i32, i64>> =
            transmute(crate::to_abi(dr));
        let t_out: &i32 = transmute((abi.methods.methods.get_t)(transmute(&abi)));
        assert_eq!(*t_out, 10);
        let u_out: &i64 = transmute((abi.methods.methods.get_u)(transmute(&abi)));
        assert_eq!(*u_out, 20);
    }
}
