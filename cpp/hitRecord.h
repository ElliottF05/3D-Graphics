#pragma once

#include "vec3.h"
#include "ray.h"
#include "material.h"

class Material;

struct HitRecord {
    Vec3 pos;
    Vec3 normal;
    std::shared_ptr<Material> material;
    float t;
    bool frontFace; 
    // if frontFace is true, then ray is hitting the outside of the object,
    // otherwise it is hitting the inside (transparent materials)

    HitRecord(Vec3 pos, Vec3 normal, float t);
    HitRecord();

    void setFaceNormal(const Ray& ray, const Vec3& outwardNormal);
};