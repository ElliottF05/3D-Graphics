#include "sphere.h"

// CONSTRUCTORS
Sphere::Sphere(Vec3 center, float radius, ObjectProperties properties) : center(center), radius(radius), properties(properties) {}
Sphere::Sphere(Vec3 center, float radius) : center(center), radius(radius) {}

// GETTERS
const Vec3& Sphere::getCenter() const {
    return center;
}
float Sphere::getRadius() const {
    return radius;
}
const ObjectProperties& Sphere::getProperties() const {
    return properties;
}

// RAY HIT
bool Sphere::rayHit(const Ray& ray, Interval hitInterval, HitRecord& hitRecord) const {
    // ray-sphere intersection code
    Vec3 oc = center - ray.getOrigin();
    auto a = ray.getDirection().lengthSquared();
    auto h = dot(ray.getDirection(), oc);
    auto c = oc.lengthSquared() - radius*radius;
    auto discriminant = h*h - a*c;

    if (discriminant < 0) {
        return false;
    }

    float sqrtd = std::sqrt(discriminant);

    float t = (h - sqrtd) / a;
    if (!hitInterval.surrounds(t)) {
        t = (h + sqrtd) / a;
        if (!hitInterval.surrounds(t)) {
            return false;
        }
    }

    hitRecord.t = t;
    hitRecord.pos = ray.at(t);
    Vec3 sphereOutwardNormal = (hitRecord.pos - center) / radius;
    hitRecord.setFaceNormal(ray, sphereOutwardNormal);

    return true;
}