#pragma once
#include <memory>
#include <string>
#include <iostream>

namespace raii_pattern_ns {

// 围绕"类文件"句柄的 RAII 包装：构造时获取，析构时释放。
class FileHandle {
public:
    FileHandle(int fd, const std::string& path)
        : fd_(fd), path_(path) {
        std::cout << "FileHandle::open fd=" << fd_ << " path=" << path_ << std::endl;
    }
    ~FileHandle() {
        std::cout << "FileHandle::close fd=" << fd_ << " path=" << path_ << std::endl;
    }
    FileHandle(const FileHandle&) = delete;
    FileHandle& operator=(const FileHandle&) = delete;

    int fd() const { return fd_; }
    const std::string& path() const { return path_; }
    long write(const std::string& data);  // 返回写入字节数
    long size() const;                     // 当前缓冲区大小

private:
    int fd_;
    std::string path_;
    long written_ = 0;
};

std::unique_ptr<FileHandle> open_file(int fd, const std::string& path);
long read_file(FileHandle& h);  // 返回可用字节数

} // namespace raii_pattern_ns
