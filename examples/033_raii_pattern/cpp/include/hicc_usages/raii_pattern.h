#pragma once
#include <cstddef>
#include <iostream>
namespace hicc_usages::raii_pattern {

class File {
public:
    static File* create(int fd);
    static void free(File* self);
    int fd() const;
    bool valid() const;
    int read(int bytes);
    int write(int bytes);
    void close();
private:
    explicit File(int fd);
    ~File();
    int fd_;
    bool closed_;
    int bytes_read_;
    int bytes_written_;
};

}  // namespace hicc_usages::raii_pattern
