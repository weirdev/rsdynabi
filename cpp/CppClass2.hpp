#pragma once

#include <algorithm>

#include "IMinAndMax.hpp"

class CppClass2 : public IMinAndMax {
    public:
        int field1;
        int field2;
        int field3;
    
    public:
        CppClass2(int field1, int field2, int field3) : field1(field1), field2(field2), field3(field3) {}
    
        int getField1() const {
            return field1;
        }

        int max() const override {
            return std::max(std::max(field1, field2), field3);
        }

        int min() const override {
            return std::min(std::min(field1, field2), field3);
        }
};