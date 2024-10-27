#include "ManualShimIMin.hpp"

extern "C" int32_t IMax_min(const IMin* iMin) {
    return iMin->min();
}
