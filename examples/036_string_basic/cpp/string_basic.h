#pragma once

#include <string>

// std::string manipulation. Key hicc pattern: MUST use import_class! to bind
// std::string — do NOT use hicc_std::string alias (memory layout incompatible,
// causes segfaults at FFI boundary).

std::string concat(const std::string& a, const std::string& b);
std::string upper(const std::string& s);
std::size_t length(const std::string& s);
