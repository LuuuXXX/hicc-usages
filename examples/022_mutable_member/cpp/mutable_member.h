#pragma once
#include <string>
#include <iostream>

namespace mutable_member_ns {

// mutable 字段允许在 const 方法中修改（典型用途：缓存、计数器）
class Query {
public:
    Query(const std::string& key) : key_(key), call_count_(0), last_result_("") {}

    const std::string& key() const { return key_; }

    // const 方法，但内部修改 mutable 字段
    std::string execute() const {
        ++call_count_;
        if (last_result_.empty()) {
            last_result_ = "[result for " + key_ + "]";
        }
        return last_result_;
    }

    int call_count() const { return call_count_; }

private:
    std::string key_;
    mutable int call_count_;
    mutable std::string last_result_;
};

} // namespace mutable_member_ns
