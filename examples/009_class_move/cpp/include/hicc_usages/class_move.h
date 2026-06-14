#pragma once
namespace hicc_usages::class_move {
class Owner {
public:
    static Owner* create(int value);
    static Owner* take_from(Owner* src);
    static void free(Owner* self);
    int get_value() const;
    bool is_valid() const;
private:
    Owner(int v);
    Owner(Owner&& other) noexcept;
    int* ptr_;
};
}
