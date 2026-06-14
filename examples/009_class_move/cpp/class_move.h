#pragma once
#include <string>
#include <iostream>
#include <utility>

namespace class_move_ns {

class Holder {
public:
    Holder() : data_(nullptr), size_(0), tag_("empty") {
        std::cout << "Holder() default" << std::endl;
    }
    explicit Holder(int sz, const std::string& tag)
        : data_(new int[sz]()), size_(sz), tag_(tag) {
        std::cout << "Holder(int,string) alloc size=" << sz << std::endl;
    }
    Holder(const Holder& other)
        : data_(new int[other.size_]), size_(other.size_), tag_(other.tag_ + "_copy") {
        for (int i = 0; i < size_; ++i) data_[i] = other.data_[i];
        std::cout << "Holder(copy)" << std::endl;
    }
    Holder(Holder&& other) noexcept
        : data_(other.data_), size_(other.size_), tag_(std::move(other.tag_) + "_moved") {
        other.data_ = nullptr;
        other.size_ = 0;
        std::cout << "Holder(move)" << std::endl;
    }
    Holder& operator=(Holder&& other) noexcept {
        if (this != &other) {
            delete[] data_;
            data_ = other.data_;
            size_ = other.size_;
            tag_ = std::move(other.tag_) + "_assigned";
            other.data_ = nullptr;
            other.size_ = 0;
        }
        return *this;
    }
    ~Holder() {
        delete[] data_;
        std::cout << "~Holder() tag=" << tag_ << std::endl;
    }

    Holder& operator+=(int delta) {
        for (int i = 0; i < size_; ++i) data_[i] += delta;
        return *this;
    }

    int size() const { return size_; }
    int first() const { return size_ > 0 ? data_[0] : -1; }
    const std::string& tag() const { return tag_; }

private:
    int* data_;
    int size_;
    std::string tag_;
};

} // namespace class_move_ns
