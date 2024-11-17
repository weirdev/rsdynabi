#include "ManualShimCppClass1.hpp"

extern "C" CppClass1* CppClass1___constructor__(int32_t field1) {
    return new CppClass1(field1);
}

extern "C" void CppClass1___destructor__(CppClass1* cppCls1) {
    delete cppCls1;
}

extern "C" int32_t CppClass1_getField1(const CppClass1* cppCls1) {
    return cppCls1->getField1();
}

extern "C" void CppClass1_printField1(const CppClass1* cppCls1) {
    cppCls1->printField1();
}
