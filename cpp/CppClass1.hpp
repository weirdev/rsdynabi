#include <iostream>

class CppClass1 {
  public:
    int field1;

  public:
    CppClass1(int field1) : field1(field1) {}

    int getField1() const {
        return field1;
    }

    void printField1() const {
        std::cout << getField1() << std::endl;
    }
};
