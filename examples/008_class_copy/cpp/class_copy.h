#pragma once
#include <string>
#include <iostream>

namespace class_copy_ns {

class Buffer {
public:
    Buffer() : size_(0), tag_("empty") {
        std::cout << "Buffer() default" << std::endl;
    }
    explicit Buffer(int sz, const std::string& tag)
        : size_(sz), tag_(tag) {
        std::cout << "Buffer(int,string) size=" << sz << " tag=" << tag << std::endl;
    }
    Buffer(const Buffer& other) : size_(other.size_), tag_(other.tag_ + "_copy") {
        std::cout << "Buffer(const Buffer&) copying size=" << size_ << std::endl;
    }
    Buffer(Buffer&& other) noexcept
        : size_(other.size_), tag_(std::move(other.tag_) + "_moved") {
        other.size_ = 0;
        std::cout << "Buffer(Buffer&&) moving size=" << size_ << std::endl;
    }
    Buffer& operator=(const Buffer& other) {
        if (this != &other) {
            size_ = other.size_;
            tag_ = other.tag_ + "_assigned";
        }
        return *this;
    }
    ~Buffer() {
        std::cout << "~Buffer() size=" << size_ << " tag=" << tag_ << std::endl;
    }

    int size() const { return size_; }
    const std::string& tag() const { return tag_; }

private:
    int size_;
    std::string tag_;
};

} // namespace class_copy_ns
