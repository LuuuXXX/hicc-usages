#include "hicc_usages/template_function.h"
// 模板定义全在 header，rust_gen 用模板语法 max_of<int>(...) 直接生成 FFI
// C++ 编译器在 hicc-build wrapper 调用时隐式实例化
namespace hicc_usages::template_function {}
