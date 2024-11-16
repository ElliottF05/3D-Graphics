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

        void fillTriangle(Vec3& v1, Vec3& v2, Vec3& v3);

    public:
        Light(Vec3 position, float thetaZ, float thetaY, float fov, int r, int g, int b, float luminosity);
        void addVerticesToShadowBuffer(std::vector<Vec3>& vertices);
        void resetShadowBuffer();
        float getLightingAmount(Vec3 worldPos, Vec3& triangleNormal);
};