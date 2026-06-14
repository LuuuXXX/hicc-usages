#pragma once
#include <string>
#include <iostream>

namespace string_basic_ns {

// Demonstrates std::string usage.
std::string greet(const std::string& name);
std::string to_upper(const std::string& s);
std::string concat(const std::string& a, const std::string& b);
size_t string_length(const std::string& s);
bool contains_substring(const std::string& hay, const std::string& needle);

} // namespace string_basic_ns
