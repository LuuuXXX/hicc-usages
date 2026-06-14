#include "class_const.h"
#include <cmath>

double Vec2::magnitude() const {
    return std::sqrt(x_ * x_ + y_ * y_);
}
