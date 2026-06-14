#pragma once
namespace hicc_usages::virtual_override {
class Base {
public:
    static Base* create();
    static void free(Base* self);
    virtual const char* name() const;
    virtual int compute(int x) const;
    virtual ~Base() = default;
};
class Derived : public Base {
public:
    static Derived* create(int multiplier);
    static void free(Derived* self);
    const char* name() const override;
    int compute(int x) const override;
    int multiplier() const;
private:
    explicit Derived(int m);
    int mult_;
};
}
