#pragma once

#include <cstdint>

#include "IMax.hpp"
#include "IMin.hpp"

class IMinAndMax : public IMin, public IMax {
  public:
    virtual ~IMinAndMax() = default;
};