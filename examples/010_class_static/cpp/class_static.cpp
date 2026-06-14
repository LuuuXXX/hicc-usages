#include "class_static.h"

namespace class_static_ns {
int Counter::s_alive_ = 0;
int Counter::s_next_id_ = 0;
const std::string Counter::s_species_ = "counter";
int Counter::s_total_created = 0;

int class_static_anchor() { return 10; }
}
