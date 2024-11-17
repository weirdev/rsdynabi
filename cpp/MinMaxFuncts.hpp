#pragma once

#include "IMax.hpp"
#include "IMin.hpp"
#include "IMinAndMax.hpp"
#include "CppClass2.hpp"
#include "CppClass3.hpp"

IMin* getMinnable() {
    return new CppClass3(1, 10);
}

IMax* getMaxable() {
    return new CppClass2(1, 2, 3);
}

IMinAndMax* getMinMaxable() {
    return new CppClass2(1, 2, 3);
}
