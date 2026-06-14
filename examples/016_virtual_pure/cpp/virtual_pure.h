#pragma once
#include <string>
#include <iostream>
#include <map>

namespace virtual_pure_ns {

// 抽象类：包含纯虚函数 = 0
class Storage {
public:
    virtual ~Storage() = default;

    // 纯虚函数
    virtual bool put(const std::string& key, const std::string& value) = 0;
    virtual std::string get(const std::string& key) const = 0;
    virtual bool remove(const std::string& key) = 0;
    virtual size_t size() const = 0;

    // 普通虚函数（带默认实现）
    virtual void dump() const {
        std::cout << "Storage size=" << size() << std::endl;
    }
};

class InMemoryStorage : public Storage {
public:
    bool put(const std::string& key, const std::string& value) override {
        data_[key] = value;
        return true;
    }
    std::string get(const std::string& key) const override {
        auto it = data_.find(key);
        return it == data_.end() ? "" : it->second;
    }
    bool remove(const std::string& key) override {
        return data_.erase(key) > 0;
    }
    size_t size() const override { return data_.size(); }
private:
    std::map<std::string, std::string> data_;
};

} // namespace virtual_pure_ns
