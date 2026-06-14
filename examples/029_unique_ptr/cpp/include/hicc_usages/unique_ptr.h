#pragma once
#include <cstddef>
#include <iostream>
#include <memory>
namespace hicc_usages::unique_ptr {

class Resource {
public:
    static Resource* create(int id);
    static void free(Resource* self);
    int id() const;
    void touch();
    int touches() const;
private:
    explicit Resource(int i);
    int id_;
    int touches_ = 0;
};

class Owner {
public:
    static Owner* create();
    static void free(Owner* self);
    void acquire(Resource* r);
    void release(int id);
    bool has(int id) const;
    std::size_t count() const;
private:
    Owner();
    ~Owner();
    struct Impl;
    std::unique_ptr<Impl> impl_;
};

}  // namespace hicc_usages::unique_ptr
