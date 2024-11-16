#pragma once

#include "vec3.h"
#include "zBuffer.h"
#include "camera.h"

class Light {
    private:
        Camera camera;
        int r, g, b;
        float luminosity;

        ZBuffer zBuffer;

    public:
        Light(Vec3 position, float thetaZ, float thetaY, float fov, int r, int g, int b, float luminosity);
        float getLightingValue(Vec3 worldPos);
};