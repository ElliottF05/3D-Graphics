#pragma once

#include "vec3.h"

class Ray {
private:
    Vec3 orig, dir;
public:
    Ray();
    Ray(const Vec3& origin, const Vec3& direction);

    const Vec3& getOrigin() const;
    const Vec3& getDirection() const;
    Vec3& getMutOrigin();
    Vec3& getMutDirection();

    Vec3 at(float t) const;
};