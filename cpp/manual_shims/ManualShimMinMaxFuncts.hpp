#pragma once

#include "cpp/IMax.hpp"
#include "cpp/IMin.hpp"
#include "cpp/IMinAndMax.hpp"

extern "C" IMin* MinMaxFuncts_getMinnable();

extern "C" IMax* MinMaxFuncts_getMaxable();

extern "C" IMinAndMax* MinMaxFuncts_getMinMaxable();
