#pragma once

#include <string>

class IMax {
  public:
    virtual ~IMax() = default;
    
    virtual int32_t max() const = 0;
};