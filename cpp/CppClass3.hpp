#pragma once

#include <vector>
#include <memory>
#include <algorithm>

#include "IMax.hpp"
#include "IMin.hpp"

class CppClass3 : public IMax, public IMin {
    public:
        std::vector<int32_t> vec;
    
    public:
        CppClass3(int32_t start, int32_t end) {
            for (int i = start; i <= end; i++) {
                vec.push_back(i);
            }
        }

        int max() const override {
            auto maxElement = std::max_element(vec.begin(), vec.end());

            if (maxElement != vec.end()) {
                return *maxElement;
            } else {
                return 0;
            }
        }

        int min() const override {
            auto minElement = std::min_element(vec.begin(), vec.end());

            if (minElement != vec.end()) {
                return *minElement;
            } else {
                return 0;
            }
        }
};