#include "virtual_override.h"

InfoLogger*  info_logger_new()   { return new InfoLogger(); }
ErrorLogger* error_logger_new()  { return new ErrorLogger(); }
void         logger_free_info(InfoLogger* l)  { delete l; }
void         logger_free_error(ErrorLogger* l) { delete l; }
