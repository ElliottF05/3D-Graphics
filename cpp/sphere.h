#pragma once

#include "hitRecord.h"
#include "vec3.h"
#include "object3D.h"
#include "ray.h"

class Sphere {
    private:
        Vec3 center;
        float radius;
        ObjectProperties properties;
    
    public:
        Sphere(Vec3 center, float radius, ObjectProperties properties);
        Sphere(Vec3 center, float radius);
        const Vec3& getCenter() const;;
        float getRadius() const;
        const ObjectProperties& getProperties() const;

        bool rayHit(const Ray& ray, float tmin, float tmax, HitRecord& hitRecord) const;
};