#include "string_basic.h"
#include <cctype>

std::string concat(const std::string& a, const std::string& b) {
    return a + b;
}

std::string upper(const std::string& s) {
    std::string out = s;
    for (char& c : out) c = static_cast<char>(std::toupper(static_cast<unsigned char>(c)));
    return out;
}

std::size_t length(const std::string& s) { return s.size(); }
