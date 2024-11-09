#include "ManualShimIMinAndMax.hpp"

extern "C" void IMinAndMax___destructor__(IMinAndMax* iMinAndMax) {
    delete iMinAndMax;
}
