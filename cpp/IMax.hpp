#pragma once

#include <cstdint>

class IMax {
  public:
    virtual ~IMax() = default;
    
    virtual int32_t max() const = 0;
};