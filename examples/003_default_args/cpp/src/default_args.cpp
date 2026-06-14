#include "hicc_usages/default_args.h"
#include <cmath>
namespace hicc_usages::default_args {
int power(int base, int exp) { return static_cast<int>(std::pow(base, exp)); }
int power(int base) { return power(base, 2); }
int power() { return power(2, 2); }
}
