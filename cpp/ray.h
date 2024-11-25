#pragma once

#include "vec3.h"

class Ray {
private:
    Vec3 orig, dir;
public:
    Ray();
    Ray(const Vec3& origin, const Vec3& direction);

    Vec3 getOrig();
    Vec3 getDir();
    Vec3& getMutOrig();
    Vec3& getMutDir();

    Vec3 at(float t);
};