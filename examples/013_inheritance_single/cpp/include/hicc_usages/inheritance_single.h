#pragma once
#include <string>
namespace hicc_usages::inheritance_single {
class Animal {
public:
    static Animal* create(const char* name);
    static void free(Animal* self);
    const char* get_name() const;
    int get_legs() const;
protected:
    explicit Animal(const char* name);
    std::string name_;
    int legs_ = 4;
};
class Dog : public Animal {
public:
    static Dog* create(const char* name);
    static void free(Dog* self);
    const char* bark() const;
private:
    explicit Dog(const char* name);
};
}
