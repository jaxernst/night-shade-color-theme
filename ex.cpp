// C++: Example of a class hierarchy with virtual functions and smart pointers
#include <iostream>
#include <memory>
#include <vector>

class Animal {
public:
    virtual void speak() const = 0; // Pure virtual function
    virtual ~Animal() {} // Virtual destructor for proper cleanup of derived types
};

class Dog : public Animal {
public:
    void speak() const override {
        std::cout << "Woof!" << std::endl;
    }
};

class Cat : public Animal {
public:
    void speak() const override {
        std::cout << "Meow!" << std::endl;
    }
};

void perform(const std::vector<std::shared_ptr<Animal>>& animals) {
    for (auto& animal : animals) {
        animal->speak();
    }
}

int main() {
    std::vector<std::shared_ptr<Animal>> animals;
    animals.push_back(std::make_shared<Dog>());
    animals.push_back(std::make_shared<Cat>());
    perform(animals);
}
