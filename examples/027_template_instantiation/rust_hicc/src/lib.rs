// Explicit instantiation: same pattern as class template — typedef in cpp!
// block, then bind as ordinary class.

hicc::cpp! {
    #include "template_instantiation.h"

    using IntStack = Stack<int>;

    inline IntStack* int_stack_new() { return new IntStack(); }
    inline void      int_stack_free(IntStack* s) { delete s; }
}

hicc::import_class! {
    #[cpp(class = "IntStack", destroy = "int_stack_free")]
    pub class IntStack {
        #[cpp(method = "void push(int)")]
        pub fn push(&mut self, v: i32);

        #[cpp(method = "int pop()")]
        pub fn pop(&mut self) -> i32;

        #[cpp(method = "bool empty() const")]
        pub fn empty(&self) -> bool;
    }
}

hicc::import_lib! {
    #![link_name = "template_instantiation_hicc"]

    #[cpp(func = "IntStack* int_stack_new()")]
    pub fn int_stack_new() -> IntStack;
}
