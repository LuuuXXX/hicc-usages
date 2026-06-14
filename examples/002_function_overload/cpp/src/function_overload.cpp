#include "hicc_usages/function_overload.h"
namespace hicc_usages::function_overload {
int add(int a, int b) { return a + b; }
double add(double a, double b) { return a + b; }
int add(int a, int b, int c) { return a + b + c; }
}
