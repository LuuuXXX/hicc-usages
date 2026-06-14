#pragma once

#include <map>
#include <string>

// Pure virtual (= abstract class). Cannot be instantiated directly; only
// the concrete derived class is exposed through FFI.

class Storage {
public:
    virtual ~Storage() = default;
    virtual std::string get(const std::string& key) const = 0;
    virtual void put(const std::string& key, const std::string& val) = 0;
    virtual int size() const = 0;
};

class MemoryStorage : public Storage {
public:
    MemoryStorage() = default;
    std::string get(const std::string& key) const override;
    void        put(const std::string& key, const std::string& val) override;
    int         size() const override { return static_cast<int>(data_.size()); }
private:
    std::map<std::string, std::string> data_;
};

MemoryStorage* mem_storage_new();
void           mem_storage_free(MemoryStorage* s);
