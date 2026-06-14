#include "hicc_usages/summary.h"
#include <cstdlib>
namespace hicc_usages::summary {

const char* status_name(Status s) {
    switch (s) {
        case Status::Ok: return "Ok";
        case Status::Error: return "Error";
        case Status::Pending: return "Pending";
    }
    return "Unknown";
}

Task::Task(int id, std::string name)
    : id_(id), name_(std::move(name)), status_(Status::Pending), priority_(0) {}
Task::~Task() = default;
Task* Task::create(int id, const char* name) {
    return new Task(id, name ? name : "");
}
void Task::free(Task* self) { delete self; }
int Task::id() const { return id_; }
const char* Task::name() const { return name_.c_str(); }
Status Task::status() const { return status_; }
void Task::set_status(Status s) { status_ = s; }
int Task::priority() const { return priority_; }
void Task::set_priority(int p) { priority_ = p; }
bool Task::is_ready() const { return status_ == Status::Ok; }

TaskQueue::TaskQueue(std::size_t cap)
    : storage_(static_cast<Task**>(std::calloc(cap, sizeof(Task*)))),
      capacity_(cap), count_(0) {}
TaskQueue::~TaskQueue() {
    for (std::size_t i = 0; i < count_; ++i) {
        Task::free(storage_[i]);
    }
    std::free(storage_);
}
TaskQueue* TaskQueue::create(std::size_t capacity) { return new TaskQueue(capacity); }
void TaskQueue::free(TaskQueue* self) { delete self; }
bool TaskQueue::push(Task* t) {
    if (count_ >= capacity_ || !t) return false;
    storage_[count_++] = t;
    return true;
}
Task* TaskQueue::pop() {
    if (count_ == 0) return nullptr;
    return storage_[--count_];
}
Task* TaskQueue::peek() const {
    if (count_ == 0) return nullptr;
    return storage_[count_ - 1];
}
std::size_t TaskQueue::size() const { return count_; }
std::size_t TaskQueue::capacity() const { return capacity_; }
int TaskQueue::total_priority() const {
    int total = 0;
    for (std::size_t i = 0; i < count_; ++i) {
        total += storage_[i]->priority();
    }
    return total;
}

int factorial(int n) {
    return n <= 1 ? 1 : n * factorial(n - 1);
}
}
