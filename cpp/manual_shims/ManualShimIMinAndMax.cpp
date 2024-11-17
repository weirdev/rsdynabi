#include "ManualShimIMinAndMax.hpp"

extern "C" void IMinAndMax___destructor__(IMinAndMax* iMinAndMax) {
    delete iMinAndMax;
}

extern "C" int32_t IMinAndMax_min(const IMinAndMax* iMinAndMax) {
    return iMinAndMax->min();
}

extern "C" int32_t IMinAndMax_max(const IMinAndMax* iMinAndMax) {
    return iMinAndMax->max();
}
