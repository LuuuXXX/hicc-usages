#include "template_instantiation.h"

namespace template_instantiation_ns {

// 显式实例化定义
template class Pair<int>;
template class Pair<std::string>;

int template_instantiation_anchor() { return 27; }
}
