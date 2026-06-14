#pragma once
#include <string>
#include <iostream>

namespace virtual_diamond_ns {

// 经典菱形继承：Device 是虚基类
class Device {
public:
    Device(const std::string& id) : id_(id) {
        std::cout << "Device(" << id_ << ")" << std::endl;
    }
    virtual ~Device() = default;
    const std::string& id() const { return id_; }
    virtual std::string category() const { return "Device"; }
protected:
    std::string id_;
};

class InputDevice : virtual public Device {
public:
    InputDevice(const std::string& id) : Device(id) {}
    std::string category() const override { return "Input"; }
    virtual int read() { return 0; }
};

class OutputDevice : virtual public Device {
public:
    OutputDevice(const std::string& id) : Device(id) {}
    std::string category() const override { return "Output"; }
    virtual void write(int v) {}
};

// 派生类只调用一次虚基类的构造
class IOCombo : public InputDevice, public OutputDevice {
public:
    IOCombo(const std::string& id)
        : Device(id), InputDevice(id), OutputDevice(id) {}
    std::string category() const override { return "IOCombo"; }
    int read() override { return last_input_; }
    void write(int v) override { last_output_ = v; }
    int last_input() const { return last_input_; }
    int last_output() const { return last_output_; }
private:
    int last_input_ = 42;
    int last_output_ = 0;
};

} // namespace virtual_diamond_ns
