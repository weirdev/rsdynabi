#include "CppClass1.hpp"

extern "C" int CppClass1_getField1(const CppClass1& cppCls1) {
    return cppCls1.getField1();
}

extern "C" void CppClass1_printField1(const CppClass1& cppCls1) {
    cppCls1.printField1();
}
