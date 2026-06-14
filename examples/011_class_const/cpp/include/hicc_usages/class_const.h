#pragma once
namespace hicc_usages::class_const {
class Value {
public:
    static Value* create(int initial);
    static void free(Value* self);
    int get() const;
    void set(int v);
    void add(int delta);
private:
    explicit Value(int v);
    int value_;
};
}
