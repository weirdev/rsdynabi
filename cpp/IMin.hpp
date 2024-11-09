#pragma once

#include <cstdint>

class IMin {
  public:
    virtual ~IMin() = default;

    virtual int32_t min() const = 0;
};