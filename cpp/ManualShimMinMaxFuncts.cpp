#include "ManualShimMinMaxFuncts.hpp"

#include "MinMaxFuncts.hpp"

extern "C" IMin* MinMaxFuncts_getMinnable() {
    return getMinnable();
}

extern "C" IMax* MinMaxFuncts_getMaxable() {
    return getMaxable();
}
