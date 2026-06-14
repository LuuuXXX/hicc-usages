#include "hicc_usages/virtual_diamond.h"
namespace hicc_usages::virtual_diamond {
int Device::count_ = 0;
int Device::count() { return count_; }
const char* Device::device_type() const { return "Device"; }
int Device::priority() const { return 0; }
int InputDevice::read() const { return 0; }
int OutputDevice::write(int) { return 0; }
IODevice::IODevice(int s) : Device(), state_(s) {}
IODevice* IODevice::create(int initial) { return new IODevice(initial); }
void IODevice::free(IODevice* self) { delete self; }
const char* IODevice::device_type() const { return "IODevice"; }
int IODevice::priority() const { return 10; }
int IODevice::read() const { return state_; }
int IODevice::write(int value) { state_ = value; return state_; }
int IODevice::state() const { return state_; }
}
