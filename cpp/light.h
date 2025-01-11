#pragma once

#include "object3D.h"
#include "vec3.h"
#include "zBuffer.h"
#include "camera.h"

class Light {
    private:
        Camera camera;
        int r, g, b;
        float luminosity;

        ZBuffer zBuffer;

        void fillTriangle(Vec3 v1, Vec3 v2, Vec3 v3);

    public:
        Light(Vec3 position, float thetaZ, float thetaY, float fov, int r, int g, int b, float luminosity);
        void addVerticesToShadowMap(const std::vector<Vec3>& vertices);
        void addObjectsToShadowMap(std::vector<Object3D>& objects);
        void resetShadowMap();
        float getLightingAmount(Vec3& worldPos, const Vec3& cameraPos, Vec3& triangleNormal, const ObjectProperties& properties);

        const ZBuffer& getZBuffer() const;
};