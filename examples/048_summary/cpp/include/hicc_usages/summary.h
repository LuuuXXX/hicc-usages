#pragma once
#include <cstddef>
#include <iostream>
#include <memory>
#include <string>
namespace hicc_usages::summary {

enum class Status : int {
    Ok = 0,
    Error = 1,
    Pending = 2,
};

const char* status_name(Status s);

class Task {
public:
    static Task* create(int id, const char* name);
    static void free(Task* self);
    int id() const;
    const char* name() const;
    Status status() const;
    void set_status(Status s);
    int priority() const;
    void set_priority(int p);
    bool is_ready() const;
private:
    Task(int id, std::string name);
    ~Task();
    int id_;
    std::string name_;
    Status status_;
    int priority_;
};

class TaskQueue {
public:
    static TaskQueue* create(std::size_t capacity);
    static void free(TaskQueue* self);
    bool push(Task* t);
    Task* pop();
    Task* peek() const;
    std::size_t size() const;
    std::size_t capacity() const;
    int total_priority() const;
private:
    TaskQueue(std::size_t cap);
    ~TaskQueue();
    Task** storage_;
    std::size_t capacity_;
    std::size_t count_;
};

int factorial(int n);

}  // namespace hicc_usages::summary
