#include "inheritance_multiple.h"
#include <iostream>

int main() {
    Sprite* s = sprite_new(16, 8);
    s->draw();
    std::cout << "byte_size=" << s->byte_size()
              << " w=" << s->width() << " h=" << s->height() << std::endl;
    sprite_free(s);
    return 0;
}
