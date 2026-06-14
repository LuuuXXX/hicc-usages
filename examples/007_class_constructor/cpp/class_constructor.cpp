#include "class_constructor.h"

Point* point_new(int x, int y)  { return new Point(x, y); }
void   point_free(Point* p)     { delete p; }
