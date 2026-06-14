#pragma once
#include <memory>
#include <cstdio>
#include <iostream>
#include <string>

namespace custom_deleter_ns {

// FILE* 风格资源 — 用自定义 deleter 调用 fclose
struct FileCloser {
    void operator()(FILE* fp) const {
        if (fp) {
            std::cout << "FileCloser closing" << std::endl;
            std::fclose(fp);
        }
    }
};

using FilePtr = std::unique_ptr<FILE, FileCloser>;

// 数组 deleter
struct ArrayDeleter {
    void operator()(int* p) const {
        std::cout << "ArrayDeleter freeing" << std::endl;
        delete[] p;
    }
};

using IntArrayPtr = std::unique_ptr<int[], ArrayDeleter>;

// 简单工厂
IntArrayPtr make_int_array(size_t n);
int read_at(const IntArrayPtr& arr, size_t i);
size_t bytes_allocated(size_t n);

// 包装：把 IntArrayPtr 装入 box-like 持有者，返回 opaque handle；用完调 destroy
struct IntArrayHandle {
    IntArrayPtr ptr;
    size_t size;
};
IntArrayHandle* make_int_array_handle(size_t n);
int handle_read_at(const IntArrayHandle* h, size_t i);
void destroy_int_array_handle(IntArrayHandle* h);
size_t handle_size(const IntArrayHandle* h);

// 不直接暴露 FilePtr（涉及 FILE* 跨 FFI 复杂，省略）
int custom_deleter_status();

} // namespace custom_deleter_ns
