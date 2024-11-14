#include "camera.h"

// CONSTRUCTORS
Camera::Camera(Vec3 pos, float thetaZ, float thetaY, float fov) : pos(pos), thetaZ(thetaZ), thetaY(thetaY), fov(fov) {}

Vec3 Camera::getPos() const {
    return pos;
}
void Camera::setPos(Vec3 pos) {
    this->pos = pos;
}
float Camera::getThetaZ() const {
    return thetaZ;
}
float Camera::getThetaY() const {
    return thetaY;
}
void Camera::setThetaY(float thetaY) {
    this->thetaY = thetaY;
}
void Camera::setThetaZ(float thetaZ) {
    this->thetaZ = thetaZ;
}
float Camera::getFov() const {
    return fov;
}
