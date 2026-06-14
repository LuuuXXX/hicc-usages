#pragma once

#include <string>
#include <utility>

// Virtual method on a concrete class. hicc treats it transparently — same
// as a regular method binding; the vtable dispatch happens on the C++ side.

class Animal {
public:
    virtual ~Animal() = default;
    virtual std::string sound() const { return "<silence>"; }
    std::string name() const { return name_; }
    explicit Animal(std::string n) : name_(std::move(n)) {}
protected:
    std::string name_;
};

class Dog : public Animal {
public:
    explicit Dog(std::string n) : Animal(std::move(n)) {}
    explicit Dog(const char* n) : Animal(n) {}
    std::string sound() const override { return "Woof!"; }
};

Dog*  dog_new(const char* name);
void  dog_free(Dog* d);
