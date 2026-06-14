#pragma once
namespace hicc_usages::class_volatile {
class Sensor {
public:
    static Sensor* create();
    static void free(Sensor* self);
    int read() const;
    void update(int v);
    int read_volatile() const volatile;
private:
    int value_ = 0;
};
}
