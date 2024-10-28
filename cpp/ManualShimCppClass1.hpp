#pragma once

#include "CppClass1.hpp"

extern "C" CppClass1* CppClass1___constructor__(int32_t field1);

extern "C" void CppClass1___destructor__(CppClass1* cppCls1);

extern "C" int32_t CppClass1_getField1(const CppClass1* cppCls1);

extern "C" void CppClass1_printField1(const CppClass1* cppCls1);
