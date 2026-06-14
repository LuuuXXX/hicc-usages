#include "custom_deleter.h"
#include <cstdio>
#include <unistd.h>

void FileClose::operator()(FileHandle* h) const { file_close(h); }

void file_close(FileHandle* h) {
    if (h) {
        // In a real impl: ::close(h->fd);
        std::printf("closing fd %d\n", h->fd);
        delete h;
    }
}

std::unique_ptr<FileHandle, FileClose> open_file(int fd) {
    return std::unique_ptr<FileHandle, FileClose>(new FileHandle{fd});
}

FileHandle* file_open(int fd) {
    return new FileHandle{fd};
}
