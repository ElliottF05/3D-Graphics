#include "ray.h"

Ray::Ray() : orig(Vec3()), dir(Vec3()) {}
Ray::Ray(const Vec3& origin, const Vec3& direction) : orig(origin), dir(direction) {}

Vec3 Ray::getOrigin() {
    return orig;
}
Vec3 Ray::getDirection() {
    return dir;
}
Vec3& Ray::getMutOrigin() {
    return orig;
}
Vec3& Ray::getMutDirection() {
    return dir;
}

Vec3 Ray::at(float t) {
    return orig + t * dir;
}