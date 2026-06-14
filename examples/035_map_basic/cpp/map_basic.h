#pragma once

#include <map>
#include <string>
#include <cstddef>

// std::map<std::string, int>. Expose insert/size/get as methods.

class StringIntMap {
public:
    StringIntMap() = default;
    void insert(const std::string& k, int v) { data_[k] = v; }
    std::size_t size() const { return data_.size(); }
    bool contains(const std::string& k) const { return data_.count(k) > 0; }
    int get_or(const std::string& k, int def) const {
        auto it = data_.find(k);
        return it == data_.end() ? def : it->second;
    }
private:
    std::map<std::string, int> data_;
};

StringIntMap* str_int_map_new();
void          str_int_map_free(StringIntMap* m);
