#pragma once
#include <memory>
#include <string>
#include <iostream>

namespace unique_ptr_ns {

class Resource {
public:
    Resource(int id, const std::string& name) : id_(id), name_(name) {
        std::cout << "Resource(" << id_ << "," << name_ << ") ctor" << std::endl;
    }
    ~Resource() {
        std::cout << "~Resource(" << id_ << "," << name_ << ") dtor" << std::endl;
    }
    int id() const { return id_; }
    const std::string& name() const { return name_; }
private:
    int id_;
    std::string name_;
};

// 返回 unique_ptr（默认 deleter）
std::unique_ptr<Resource> make_resource(int id, const std::string& name);

// 接管 unique_ptr（消费）
int consume_resource(std::unique_ptr<Resource> r);

} // namespace unique_ptr_ns
