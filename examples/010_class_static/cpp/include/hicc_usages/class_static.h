#pragma once
namespace hicc_usages::class_static {
class Counter {
public:
    static Counter* create();
    static void free(Counter* self);
    static int get_instance_count();
    void tick();
    int get_ticks() const;
private:
    static int instance_count_;
    int ticks_ = 0;
};
}
