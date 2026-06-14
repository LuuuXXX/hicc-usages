#pragma once

// enum class is not directly nameable through hicc's `#[cpp(...)]` attribute
// (only POD-like scalar types cross the FFI). We pass enum values as `int`
// and provide factory/describe functions on the C++ side.

enum class Color : int { Red = 0, Green = 1, Blue = 2 };

int color_to_int(Color c);
Color int_to_color(int v);
const char* color_name(Color c);

// Convenience wrappers used by FFI: only int crosses the boundary.
int to_int_red();
int to_int_green();
int to_int_blue();
const char* color_name_for_int(int v);
