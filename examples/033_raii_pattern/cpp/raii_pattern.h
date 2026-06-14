#pragma once

// RAII: constructor acquires, destructor releases. From Rust's perspective
// the binding looks like a normal class — Drop fires the destroy function.

class Lock {
public:
    explicit Lock(int id) : id_(id), locked_(true) {}
    ~Lock() { if (locked_) release(); }
    void release() { locked_ = false; }
    bool is_locked() const { return locked_; }
    int  id() const { return id_; }
private:
    int  id_;
    bool locked_;
};

Lock* lock_new(int id);
void  lock_free(Lock* l);
