#pragma once

#include "vec3.h"
#include "ray.h"

struct HitRecord {
    Vec3 pos;
    Vec3 normal;
    float t;
    bool frontFace; 
    // if frontFace is true, then ray is hitting the outside of the object,
    // otherwise it is hitting the inside (transparent materials)

    HitRecord(Vec3 pos, Vec3 normal, float t);
    HitRecord();

    void setFaceNormal(const Ray& ray, const Vec3& outwardNormal);
};