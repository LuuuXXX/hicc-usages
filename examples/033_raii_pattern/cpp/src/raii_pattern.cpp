#include "hicc_usages/raii_pattern.h"
namespace hicc_usages::raii_pattern {
File::File(int fd) : fd_(fd), closed_(false), bytes_read_(0), bytes_written_(0) {}
File::~File() { close(); }
File* File::create(int fd) { return new File(fd); }
void File::free(File* self) { delete self; }
int File::fd() const { return fd_; }
bool File::valid() const { return !closed_ && fd_ >= 0; }
int File::read(int bytes) { bytes_read_ += bytes; return bytes; }
int File::write(int bytes) { bytes_written_ += bytes; return bytes; }
void File::close() { closed_ = true; }
}
