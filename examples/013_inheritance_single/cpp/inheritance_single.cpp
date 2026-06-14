#include "inheritance_single.h"

Square* square_new(int side) { return new Square(side); }
void    square_free(Square* s) { delete s; }
