#include "inheritance_multiple.h"

Sprite* sprite_new(int w, int h) { return new Sprite(w, h); }
void    sprite_free(Sprite* s)   { delete s; }
