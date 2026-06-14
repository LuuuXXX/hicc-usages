#pragma once
#include <map>
#include <string>
#include <iostream>

namespace map_basic_ns {

// Demonstrates std::map<int, std::string>.
void put(std::map<int, std::string>& m, int key, const std::string& val);
std::string get_or(const std::map<int, std::string>& m, int key, const std::string& def);
size_t map_size(const std::map<int, std::string>& m);
long sum_key_values(const std::map<int, std::string>& m);

} // namespace map_basic_ns
