#pragma once
#include <string>
#include <iostream>

namespace inheritance_single_ns {

class Animal {
public:
    Animal(const std::string& name) : name_(name) {
        std::cout << "Animal(" << name_ << ")" << std::endl;
    }
    virtual ~Animal() {
        std::cout << "~Animal(" << name_ << ")" << std::endl;
    }

    const std::string& name() const { return name_; }
    virtual std::string sound() const { return "?"; }
    virtual int legs() const = 0;

protected:
    std::string name_;
};

class Dog : public Animal {
public:
    Dog(const std::string& name) : Animal(name) {}
    std::string sound() const override { return "Woof"; }
    int legs() const override { return 4; }
    std::string breed() const { return "Unknown"; }
};

class Cat : public Animal {
public:
    Cat(const std::string& name) : Animal(name) {}
    std::string sound() const override { return "Meow"; }
    int legs() const override { return 4; }
};

} // namespace inheritance_single_ns
