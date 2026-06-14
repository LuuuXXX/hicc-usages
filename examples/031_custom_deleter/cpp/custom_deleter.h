#pragma once

#include <memory>

// Custom deleter: unique_ptr<T, Deleter> calls a user-provided deleter
// instead of `delete`. We expose the deleter as a free function for hicc's
// destroy= attribute.

struct FileHandle {
    int fd;
};

struct FileClose {
    void operator()(FileHandle* h) const;  // defined in .cpp
};

void file_close(FileHandle* h);  // free-function version for destroy=

std::unique_ptr<FileHandle, FileClose> open_file(int fd);
FileHandle* file_open(int fd);
