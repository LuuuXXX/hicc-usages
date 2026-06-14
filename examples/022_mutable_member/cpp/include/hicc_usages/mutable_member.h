#pragma once
namespace hicc_usages::mutable_member {
class Cache {
public:
    static Cache* create();
    static void free(Cache* self);
    int get_value(int key) const;
    void set_value(int key, int value);
    int access_count() const;
private:
    mutable int access_count_ = 0;
    int values_[16] = {0};
};
}
