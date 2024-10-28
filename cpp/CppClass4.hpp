#pragma once

#include "CppClass2.hpp"

class CppClass4 : public CppClass2 {
    public:
        int field4;
    
    public:
        CppClass4(int field1, int field2, int field3, int field4) : 
                CppClass2(field1, field2, field3), 
                field4(field4) {}

        int max() const override {
            return std::max(CppClass2::max(), field4);
        }

        int min() const override {
            return std::min(CppClass2::min(), field4);
        }
};