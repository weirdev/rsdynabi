#include <iostream>

class CppClass1 {
  public:
    int field1;

  public:
    CppClass1(int field1) : field1(field1) {}

    int getField1() {
        return field1;
    }

    void printField1() {
        std::cout << getField1() << std::endl;
    }
};
