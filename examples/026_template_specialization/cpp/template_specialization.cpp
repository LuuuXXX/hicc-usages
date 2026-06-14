#include "template_specialization.h"

const char* type_name_int()         { return TypeName<int>::get(); }
const char* type_name_bool()        { return TypeName<bool>::get(); }
const char* type_name_generic()     { return TypeName<double>::get(); }
