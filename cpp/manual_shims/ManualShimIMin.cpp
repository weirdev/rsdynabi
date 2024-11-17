#include "ManualShimIMin.hpp"

extern "C" void IMin___destructor__(IMin* iMin) {
    delete iMin;
}

extern "C" int32_t IMin_min(const IMin* iMin) {
    return iMin->min();
}
