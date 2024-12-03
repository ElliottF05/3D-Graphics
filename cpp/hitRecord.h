#pragma once

#include "vec3.h"

struct HitRecord {
    Vec3 pos;
    Vec3 normal;
    float t;

    HitRecord(Vec3 pos, Vec3 normal, float t);
    HitRecord();
};