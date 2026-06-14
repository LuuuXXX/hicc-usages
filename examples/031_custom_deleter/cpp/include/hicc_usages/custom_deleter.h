#pragma once
#include <cstddef>
#include <iostream>
#include <memory>
namespace hicc_usages::custom_deleter {

class FileHandle {
public:
    static FileHandle* create(int fd);
    static void free(FileHandle* self);
    int fd() const;
    bool closed() const;
    void close();
private:
    explicit FileHandle(int fd);
    int fd_;
    bool closed_;
};

class FileManager {
public:
    struct Impl;
    static FileManager* create();
    static void free(FileManager* self);
    void open(int fd);
    bool close(int fd);
    std::size_t open_count() const;
private:
    FileManager();
    ~FileManager();
    std::unique_ptr<Impl, void(*)(Impl*)> impl_;
};

}  // namespace hicc_usages::custom_deleter
