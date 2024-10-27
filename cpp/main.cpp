#include <iostream>

#include "CppClass1.hpp"

int main() {
    std::cout << "Cpp only" << std::endl;
    CppClass1 cppCls1(10);
    cppCls1.printField1();
    return 0;
}