#include "raii_pattern.h"

Lock* lock_new(int id) { return new Lock(id); }
void  lock_free(Lock* l) { delete l; }   // ~Lock() runs, releases
