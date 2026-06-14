#include "raii_pattern.h"

namespace raii_pattern_ns {

long FileHandle::write(const std::string& data) {
    written_ += static_cast<long>(data.size());
    std::cout << "FileHandle::write fd=" << fd_ << " bytes=" << data.size() << std::endl;
    return static_cast<long>(data.size());
}

long FileHandle::size() const {
    return written_;
}

std::unique_ptr<FileHandle> open_file(int fd, const std::string& path) {
    return std::make_unique<FileHandle>(fd, path);
}

long read_file(FileHandle& h) {
    return h.size();
}

int raii_pattern_anchor() { return 33; }

} // namespace raii_pattern_ns
