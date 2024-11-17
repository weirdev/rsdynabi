#pragma once

#include "cpp/IMinAndMax.hpp"

extern "C" void IMinAndMax___destructor__(IMinAndMax* iMinAndMax);

// NOTE: Any reason to not recursively codegen inherited methods? (what we are simulating here)
// as opposed to having the rust side call the inherited methods directly?

extern "C" int32_t IMinAndMax_min(const IMinAndMax* iMinAndMax);

extern "C" int32_t IMinAndMax_max(const IMinAndMax* iMinAndMax);
