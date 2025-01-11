#include "hitRecord.h"

// CONSTRUCTORS
HitRecord::HitRecord(Vec3 pos, Vec3 normal, float t) : pos(pos), normal(normal), t(t) {}
HitRecord::HitRecord() : pos(Vec3()), normal(Vec3()), t(0) {}

void HitRecord::setFaceNormal(const Ray &ray, const Vec3 &outwardNormal) {
    frontFace = dot(ray.getDirection(), outwardNormal) < 0; // if ray and normal are in opposite directions (dot < 0), then ray is hitting the front face of the object
    normal = frontFace ? outwardNormal : outwardNormal * -1; // this flips the [local reflection] normal if the ray hits from the inside
}