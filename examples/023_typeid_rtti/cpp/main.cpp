#include "typeid_rtti.h"

int main() {
    using namespace typeid_rtti_ns;
    DerivedA a;
    DerivedB b;
    Base& ra = a;
    Base& rb = b;
    std::cout << "ra.name=" << ra.name() << std::endl;
    std::cout << "ra.typeid.name=" << type_name_base(ra) << std::endl;
    std::cout << "same_type(a,a)=" << same_type(ra, ra) << std::endl;
    std::cout << "same_type(a,b)=" << same_type(ra, rb) << std::endl;
    std::cout << "is_derived_a(a)=" << is_derived_a(ra) << std::endl;
    return 0;
}
