#include "constexpr_basic.h"

namespace constexpr_basic_ns {

double compute_area(double radius) {
    Circle c(radius);
    return c.area();
}

const double& get_pi() { return Constants::PI; }
const int& get_buffer_size() { return Constants::BUFFER_SIZE; }
const long& get_big_number() { return Constants::BIG_NUMBER; }

int constexpr_basic_anchor() { return 46; }

} // namespace constexpr_basic_ns
