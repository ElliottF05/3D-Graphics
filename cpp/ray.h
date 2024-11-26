#pragma once

#include "vec3.h"

class Ray {
private:
    Vec3 orig, dir;
public:
    Ray();
    Ray(const Vec3& origin, const Vec3& direction);

    Vec3 getOrigin();
    Vec3 getDirection();
    Vec3& getMutOrigin();
    Vec3& getMutDirection();

    Vec3 at(float t);
};