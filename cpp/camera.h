#pragma once

#include "vec3.h"

class Camera {
    private:
        Vec3 pos;
        float thetaZ, thetaY, fov, focusDistance, defocusAngle;
    public:
        Camera(Vec3 pos, float thetaZ, float thetaY, float fov, float focusDistance, float defocusAngle);
        Camera(Vec3 pos, float thetaZ, float thetaY, float fov);
        Camera();

        Vec3 getPos() const;
        void setPos(Vec3 pos);

        float getThetaZ() const;
        float getThetaY() const;
        void setThetaY(float thetaY);
        void setThetaZ(float thetaZ);
        void lookInDirection(const Vec3& direction);
        void lookAt(const Vec3& target);
        float getFov() const;
        void setFov(float fov);
        void setFovDegrees(float fovDegrees);

        float getFocusDistance() const;
        void setFocusDistance(float focusDistance);
        float getDefocusAngle() const;
        void setDefocusAngle(float defocusAngle);
        void setDefocusAngleDegrees(float defocusAngleDegrees);
};