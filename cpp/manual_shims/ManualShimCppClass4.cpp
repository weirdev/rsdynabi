#pragma once

#include "ManualShimCppClass4.hpp"

extern "C" CppClass4* CppClass4___constructor__(int32_t field1, int32_t field2, int32_t field3, int32_t field4) {
    return new CppClass4(field1, field2, field3, field4);
}

extern "C" void CppClass4___destructor__(CppClass4* cppCls4) {
    delete cppCls4;
}

extern "C" int32_t CppClass4_max(const CppClass4* cppCls4) {
    return cppCls4->max();
}

extern "C" int32_t CppClass4_min(const CppClass4* cppCls4) {
    return cppCls4->min();
}
