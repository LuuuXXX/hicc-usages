#pragma once
namespace hicc_usages::explicit_ctor {
class Distance {
public:
    static Distance* create_from_meters(int m);
    static Distance* create_from_feet(int f);
    static void free(Distance* self);
    int meters() const;
    int feet() const;
private:
    explicit Distance(int meters);
    int meters_;
};
}
