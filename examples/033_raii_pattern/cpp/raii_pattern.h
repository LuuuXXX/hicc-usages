#pragma once
#include <memory>
#include <string>
#include <iostream>

namespace raii_pattern_ns {

// RAII wrapper around a "file-like" handle: acquires on ctor, releases on dtor.
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
    long write(const std::string& data);  // returns bytes written
    long size() const;                     // current buffer size

private:
    int fd_;
    std::string path_;
    long written_ = 0;
};

std::unique_ptr<FileHandle> open_file(int fd, const std::string& path);
long read_file(FileHandle& h);  // returns bytes available

} // namespace raii_pattern_ns
