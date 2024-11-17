#pragma once

#include "IMax.hpp"
#include "IMin.hpp"
#include "IMinAndMax.hpp"

extern "C" IMin* MinMaxFuncts_getMinnable();

extern "C" IMax* MinMaxFuncts_getMaxable();

extern "C" IMinAndMax* MinMaxFuncts_getMinMaxable();
