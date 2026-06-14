#include "operator_overload.h"

Vec2 vec2_add(const Vec2& a, const Vec2& b) { return a + b; }
Vec2 vec2_sub(const Vec2& a, const Vec2& b) { return a - b; }
bool vec2_eq(const Vec2& a, const Vec2& b)  { return a == b; }

Vec2* vec2_new(int x, int y) { return new Vec2(x, y); }
void  vec2_free(Vec2* v)     { delete v; }
