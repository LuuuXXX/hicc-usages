#include "hicc_usages/custom_deleter.h"
#include <map>
#include <memory>
namespace hicc_usages::custom_deleter {

struct FileManager::Impl {
    std::map<int, FileHandle*> handles;
    ~Impl() {
        for (auto& [k, h] : handles) {
            h->close();
            FileHandle::free(h);
        }
    }
};

FileHandle::FileHandle(int fd) : fd_(fd), closed_(false) {}
FileHandle* FileHandle::create(int fd) { return new FileHandle(fd); }
void FileHandle::free(FileHandle* self) { delete self; }
int FileHandle::fd() const { return fd_; }
bool FileHandle::closed() const { return closed_; }
void FileHandle::close() { closed_ = true; }

static void file_manager_deleter(FileManager::Impl* p) { delete p; }

FileManager::FileManager()
    : impl_(new Impl(), file_manager_deleter) {}
FileManager::~FileManager() = default;
FileManager* FileManager::create() { return new FileManager(); }
void FileManager::free(FileManager* self) { delete self; }
void FileManager::open(int fd) {
    impl_->handles[fd] = FileHandle::create(fd);
}
bool FileManager::close(int fd) {
    auto it = impl_->handles.find(fd);
    if (it == impl_->handles.end()) return false;
    it->second->close();
    FileHandle::free(it->second);
    impl_->handles.erase(it);
    return true;
}
std::size_t FileManager::open_count() const { return impl_->handles.size(); }

}
