#include "ManualShimIMax.hpp"

extern "C" void IMax___destructor__(IMax* iMax) {
    delete iMax;
}

extern "C" int32_t IMax_max(const IMax* iMax) {
    return iMax->max();
}
