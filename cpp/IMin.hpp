#pragma once

#include <string>

class IMin {
  public:
    virtual ~IMin() = default;

    virtual int32_t min() const = 0;
};