//! 自动生成：hicc_usage_summary
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/summary.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::summary::Task", destroy = "hicc_usages::summary::Task::free")]
    pub class Task {
        #[cpp(method = "int id() const")]
        pub fn id(&self) -> i32;
        #[cpp(method = "const char * name() const")]
        pub fn name(&self) -> *const i8;
        #[cpp(method = "int priority() const")]
        pub fn priority(&self) -> i32;
        #[cpp(method = "void set_priority(int)")]
        pub fn set_priority(&mut self, p: i32) -> ();
        #[cpp(method = "bool is_ready() const")]
        pub fn is_ready(&self) -> bool;
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::summary::TaskQueue", destroy = "hicc_usages::summary::TaskQueue::free")]
    pub class TaskQueue {
        #[cpp(method = "int total_priority() const")]
        pub fn total_priority(&self) -> i32;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_summary_adapter"]
    pub class Task;
    pub class TaskQueue;
    #[cpp(func = "hicc_usages::summary::Task * hicc_usages::summary::Task::create(int, const char *)")]
    pub fn task_new(id: i32, name: *const i8) -> Task;
    #[cpp(func = "hicc_usages::summary::TaskQueue * hicc_usages::summary::TaskQueue::create(std::size_t)")]
    pub fn taskqueue_new(capacity: usize) -> TaskQueue;
    #[cpp(func = "int hicc_usages::summary::factorial(int)")]
    pub fn factorial(n: i32) -> i32;
}
