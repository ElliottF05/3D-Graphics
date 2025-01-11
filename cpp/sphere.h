#pragma once

#include "hitRecord.h"
#include "interval.h"
#include "material.h"
#include "vec3.h"
#include "object3D.h"
#include "ray.h"
#include <memory>

class Sphere {
    private:
        Vec3 center;
        float radius;
        std::shared_ptr<Material> material;
    
    public:
        Sphere(Vec3 center, float radius, std::shared_ptr<Material> material);
        Sphere(Vec3 center, float radius);
        const Vec3& getCenter() const;;
        float getRadius() const;
        const std::shared_ptr<Material> getProperties() const;

        bool rayHit(const Ray& ray, Interval hitInterval, HitRecord& hitRecord) const;
};