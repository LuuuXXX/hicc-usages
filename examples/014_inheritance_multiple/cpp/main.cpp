#include "inheritance_multiple.h"

int main() {
    using namespace inheritance_multiple_ns;
    Circle c(5.0f);
    Square s(3.0f);

    c.draw();
    s.draw();
    std::cout << "circle shape=" << c.shape() << " bytes=" << c.bytes() << std::endl;
    std::cout << "square serialize=" << s.serialize() << std::endl;

    Drawable* d1 = &c;
    Serializable* s2 = &c;
    std::cout << "as Drawable: " << d1->shape() << std::endl;
    std::cout << "as Serializable: " << s2->serialize() << std::endl;
    return 0;
}
