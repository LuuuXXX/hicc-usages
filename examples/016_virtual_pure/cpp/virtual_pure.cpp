#include "virtual_pure.h"
#include <map>

std::string MemoryStorage::get(const std::string& key) const {
    auto it = MemoryStorage::data_.find(key);
    return it == MemoryStorage::data_.end() ? "" : it->second;
}

void MemoryStorage::put(const std::string& key, const std::string& val) {
    MemoryStorage::data_[key] = val;
}

MemoryStorage* mem_storage_new() { return new MemoryStorage(); }
void           mem_storage_free(MemoryStorage* s) { delete s; }
