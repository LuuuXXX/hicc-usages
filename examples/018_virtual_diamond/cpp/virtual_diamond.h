#pragma once

// Diamond inheritance with virtual base — hicc has no direct support.
// Limitation: rather than try to model the diamond, we simplify to a
// single concrete class that combines the behavior of the two middle tiers
// (a common pragmatic refactoring when crossing the FFI boundary).

class Device {
public:
    virtual ~Device() = default;
    virtual int priority() const = 0;
};

class InputDevice : virtual public Device {
public:
    virtual int read() = 0;
};

class OutputDevice : virtual public Device {
public:
    virtual void write(int v) = 0;
};

// Combined: collapses InputDevice + OutputDevice + Device into one concrete
// class. This is the simplification documented in README.
class Console : public InputDevice, public OutputDevice {
public:
    Console() : value_(0) {}
    int  priority() const override { return 5; }
    int  read() override { int v = value_; value_ = 0; return v; }
    void write(int v) override { value_ = v; }
private:
    int value_;
};

Console* console_new();
void     console_free(Console* c);
