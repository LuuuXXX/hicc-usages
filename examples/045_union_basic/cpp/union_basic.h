#pragma once
#include <string>
#include <iostream>

namespace union_basic_ns {

// POD union —— trivially copyable，无用户自定义构造/析构
union Value {
    int i;
    float f;
    long l;
};

// 标签：描述当前活跃的 union 成员
enum class Tag : int { Int = 0, Float = 1, Long = 2 };

// 辅助函数：访问器 + 构造器（hicc 无法直接 FFI union 字段）
inline int value_as_int(Value v) { return v.i; }
inline float value_as_float(Value v) { return v.f; }
inline long value_as_long(Value v) { return v.l; }

inline Value make_value_int(int x) { Value v; v.i = x; return v; }
inline Value make_value_float(float x) { Value v; v.f = x; return v; }
inline Value make_value_long(long x) { Value v; v.l = x; return v; }

// 包含 union 的类
class Box {
public:
    Box(int x);
    Box(float x);
    Box(long x);
    Box(const Box& other);

    Tag tag() const;
    int as_int() const;
    float as_float() const;
    long as_long() const;

    void set_int(int x);
    void set_float(float x);
    void set_long(long x);

    std::string describe() const;
private:
    Value v_;
    Tag tag_;
};

Box make_box_int(int x);
Box make_box_float(float x);
Box make_box_long(long x);

} // namespace union_basic_ns
