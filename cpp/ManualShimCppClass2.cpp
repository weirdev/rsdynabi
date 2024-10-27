#include "ManualShimCppClass2.hpp"

extern "C" CppClass2* CppClass2___constructor__(int32_t field1, int32_t field2, int32_t field3) {
    return new CppClass2(field1, field2, field3);
}

extern "C" void CppClass2___destructor__(CppClass2* cppCls2) {
    delete cppCls2;
}

extern "C" int32_t CppClass2_getField1(const CppClass2* cppCls2) {
    return cppCls2->getField1();
}
