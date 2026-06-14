#include "hicc_usages/summary.h"
#include <cassert>
#include <cstring>
#include <iostream>
int main() {
    using namespace hicc_usages::summary;
    assert(strcmp(status_name(Status::Ok), "Ok") == 0);

    Task* t1 = Task::create(1, "Task A");
    Task* t2 = Task::create(2, "Task B");
    t1->set_priority(5);
    t2->set_priority(10);
    t2->set_status(Status::Ok);
    assert(!t1->is_ready());
    assert(t2->is_ready());

    TaskQueue* q = TaskQueue::create(10);
    q->push(t1);
    q->push(t2);
    assert(q->size() == 2);
    assert(q->total_priority() == 15);
    assert(q->peek() == t2);
    q->pop();  // pops t2; t2 ownership transferred
    Task::free(t2);
    assert(q->size() == 1);

    TaskQueue::free(q);  // frees remaining t1 too
    assert(factorial(6) == 720);
    std::cout << "[summary] C++ test OK" << std::endl;
    return 0;
}
