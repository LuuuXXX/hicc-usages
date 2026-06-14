//! 自动生成：hicc_usage_virtual_diamond
//!
//! 由 tools/rust-gen/rust_gen.py 从 ../ast/symbols.json 生成。
//! 请勿手动编辑；如需调整请修改 tools/rust-gen/special.yaml 后重新生成。

hicc::cpp! {
    #include "hicc_usages/virtual_diamond.h"
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::virtual_diamond::Device")]
    pub class Device {
        #[cpp(method = "const char * device_type() const")]
        pub fn device_type(&self) -> *const i8;
        #[cpp(method = "int priority() const")]
        pub fn priority(&self) -> i32;
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::virtual_diamond::InputDevice")]
    pub class InputDevice {
        #[cpp(method = "int read() const")]
        pub fn read(&self) -> i32;
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::virtual_diamond::OutputDevice")]
    pub class OutputDevice {
        #[cpp(method = "int write(int)")]
        pub fn write(&mut self, value: i32) -> i32;
    }
}
hicc::import_class! {
    #[cpp(class = "hicc_usages::virtual_diamond::IODevice", destroy = "hicc_usages::virtual_diamond::IODevice::free")]
    pub class IODevice {
        #[cpp(method = "const char * device_type() const")]
        pub fn device_type(&self) -> *const i8;
        #[cpp(method = "int priority() const")]
        pub fn priority(&self) -> i32;
        #[cpp(method = "int read() const")]
        pub fn read(&self) -> i32;
        #[cpp(method = "int write(int)")]
        pub fn write(&mut self, value: i32) -> i32;
        #[cpp(method = "int state() const")]
        pub fn state(&self) -> i32;
    }
}
hicc::import_lib! {
    #![link_name = "hicc_usage_virtual_diamond_adapter"]
    pub class Device;
    pub class InputDevice;
    pub class OutputDevice;
    pub class IODevice;
    #[cpp(func = "int hicc_usages::virtual_diamond::Device::count()")]
    pub fn device_count() -> i32;
    #[cpp(func = "hicc_usages::virtual_diamond::IODevice * hicc_usages::virtual_diamond::IODevice::create(int)")]
    pub fn iodevice_new(initial: i32) -> IODevice;
}
