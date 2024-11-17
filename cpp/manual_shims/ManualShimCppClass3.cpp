#include "ManualShimCppClass3.hpp"

extern "C" CppClass3* CppClass3___constructor__(int32_t start, int32_t end) {
    return new CppClass3(start, end);
}

extern "C" void CppClass3___destructor__(CppClass3* cppCls2) {
    delete cppCls2;
}

extern "C" int32_t CppClass3_max(const CppClass3* cppCls2) {
    return cppCls2->max();
}

extern "C" int32_t CppClass3_min(const CppClass3* cppCls2) {
    return cppCls2->min();
}
