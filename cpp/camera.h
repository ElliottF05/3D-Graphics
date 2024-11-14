#pragma once

#include "vec3.h"

class Camera {
    private:
        Vec3 pos;
        float thetaZ, thetaY, fov;
    public:
        Camera(Vec3 pos, float thetaZ, float thetaY, float fov);
        Camera();

        Vec3 getPos() const;
        void setPos(Vec3 pos);

        float getThetaZ() const;
        float getThetaY() const;
        void setThetaY(float thetaY);
        void setThetaZ(float thetaZ);
        float getFov() const;
};