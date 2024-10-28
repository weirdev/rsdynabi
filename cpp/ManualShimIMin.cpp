#include "ManualShimIMin.hpp"

extern "C" int32_t IMin_min(const IMin* iMin) {
    return iMin->min();
}
