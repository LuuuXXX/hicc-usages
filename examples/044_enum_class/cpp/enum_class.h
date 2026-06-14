#pragma once
#include <string>
#include <iostream>

namespace enum_class_ns {

// 主 enum class
enum class Color { Red, Green, Blue };

// 第二个 enum class，带显式值
enum class Status : int { Active = 10, Inactive = 20, Pending = 30 };

// enum 与 int 相互转换的包装（hicc 无法直接 FFI enum class）
inline int color_to_int(Color c) { return static_cast<int>(c); }
inline Color color_from_int(int v) {
    if (v < 0 || v > 2) return Color::Red;  // 安全默认值
    return static_cast<Color>(v);
}
inline int status_to_int(Status s) { return static_cast<int>(s); }
inline Status status_from_int(int v) {
    if (v == 10) return Status::Active;
    if (v == 20) return Status::Inactive;
    if (v == 30) return Status::Pending;
    return Status::Pending;
}

// 字符串辅助函数
std::string color_name(Color c);
Color color_parse(const std::string& s);

// 接收/返回 enum 的类
class Light {
public:
    Light(Color initial);
    Color current() const;
    void set(Color c);
    int brightness() const;       // 返回由 Color 派生的 int（Red=100，Green=200，Blue=300）
private:
    Color color_;
};

Light make_light(Color initial);

} // namespace enum_class_ns
