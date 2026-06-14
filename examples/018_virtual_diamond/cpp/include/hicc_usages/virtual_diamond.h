#pragma once
#include <iostream>
namespace hicc_usages::virtual_diamond {
class Device {
public:
    static int count();
    virtual const char* device_type() const;
    virtual int priority() const;
    virtual ~Device() = default;
protected:
    Device() { ++count_; }
    static int count_;
};
class InputDevice : virtual public Device {
public:
    virtual int read() const;
};
class OutputDevice : virtual public Device {
public:
    virtual int write(int value);
};
class IODevice : public InputDevice, public OutputDevice {
public:
    static IODevice* create(int initial);
    static void free(IODevice* self);
    const char* device_type() const override;
    int priority() const override;
    int read() const override;
    int write(int value) override;
    int state() const;
private:
    explicit IODevice(int s);
    int state_;
};
}
