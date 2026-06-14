#pragma once
#include <cstddef>
#include <iostream>
#include <string>
namespace hicc_usages::string_basic {

class StringBuf {
public:
    static StringBuf* create();
    static StringBuf* create_from(const char* s);
    static void free(StringBuf* self);
    void append(const char* s);
    const char* c_str() const;
    std::size_t length() const;
    bool equals(const char* s) const;
    int find(const char* needle) const;
    const char* substring(std::size_t start, std::size_t len) const;
private:
    StringBuf();
    explicit StringBuf(const char* s);
    ~StringBuf();
    std::string data_;
    mutable std::string substring_cache_;
};

}  // namespace hicc_usages::string_basic
