#include "string_basic.h"
#include <algorithm>
#include <cctype>

namespace string_basic_ns {

std::string greet(const std::string& name) {
    return "hello, " + name + "!";
}

std::string to_upper(const std::string& s) {
    std::string out = s;
    for (auto& c : out) c = static_cast<char>(std::toupper(static_cast<unsigned char>(c)));
    return out;
}

std::string concat(const std::string& a, const std::string& b) {
    return a + b;
}

size_t string_length(const std::string& s) {
    return s.size();
}

bool contains_substring(const std::string& hay, const std::string& needle) {
    if (needle.empty()) return true;
    return hay.find(needle) != std::string::npos;
}

int string_basic_anchor() { return 36; }

} // namespace string_basic_ns
