#include "hicc_usages/string_basic.h"
#include <cstring>
namespace hicc_usages::string_basic {
StringBuf::StringBuf() : data_() {}
StringBuf::StringBuf(const char* s) : data_(s ? s : "") {}
StringBuf::~StringBuf() = default;
StringBuf* StringBuf::create() { return new StringBuf(); }
StringBuf* StringBuf::create_from(const char* s) { return new StringBuf(s); }
void StringBuf::free(StringBuf* self) { delete self; }
void StringBuf::append(const char* s) { if (s) data_.append(s); }
const char* StringBuf::c_str() const { return data_.c_str(); }
std::size_t StringBuf::length() const { return data_.size(); }
bool StringBuf::equals(const char* s) const {
    if (!s) return false;
    return data_ == s;
}
int StringBuf::find(const char* needle) const {
    if (!needle) return -1;
    auto pos = data_.find(needle);
    return pos == std::string::npos ? -1 : static_cast<int>(pos);
}
const char* StringBuf::substring(std::size_t start, std::size_t len) const {
    substring_cache_ = data_.substr(start, len);
    return substring_cache_.c_str();
}
}
