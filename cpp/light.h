#pragma once

#include "vec3.h"

class Light {
    private:
        Vec3 position;
        int r, g, b;
        float luminosity;
    public:
        Light();
};