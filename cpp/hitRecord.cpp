#include "hitRecord.h"

HitRecord::HitRecord(Vec3 pos, Vec3 normal, float t) : pos(pos), normal(normal), t(t) {}
HitRecord::HitRecord() : pos(Vec3()), normal(Vec3()), t(0) {}