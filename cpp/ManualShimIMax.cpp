#include "ManualShimIMax.hpp"

extern "C" int32_t IMax_max(const IMax* iMax) {
    return iMax->max();
}
