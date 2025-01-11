#include "camera.h"
#include "utils.h"

// CONSTRUCTORS
Camera::Camera(Vec3 pos, float thetaZ, float thetaY, float fov, float focusDistance, float defocusAngle) : pos(pos), thetaZ(thetaZ), thetaY(thetaY), fov(fov), focusDistance(focusDistance), defocusAngle(defocusAngle) {}
Camera::Camera(Vec3 pos, float thetaZ, float thetaY, float fov) : pos(pos), thetaZ(thetaZ), thetaY(thetaY), fov(fov) {}
Camera::Camera() : pos(Vec3()), thetaZ(0), thetaY(0), fov(M_PI / 2.0f), focusDistance(0), defocusAngle(0) {}

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
    if (thetaY < -M_PI / 2.0) {
        this->thetaY = -M_PI / 2.0;
    }
    if (thetaY > M_PI / 2.0) {
        this->thetaY = M_PI / 2.0;
    }
}
void Camera::setThetaZ(float thetaZ) {
    this->thetaZ = thetaZ;
}
void Camera::lookInDirection(const Vec3 &direction) {
    thetaZ = std::atan2(direction.y, direction.x);
    thetaY = std::asin(direction.z / direction.length());
}
void Camera::lookAt(const Vec3 &target) {
    Vec3 dir = target - pos;
    lookInDirection(dir);
}
float Camera::getFov() const {
    return fov;
}
void Camera::setFov(float fov) {
    this->fov = fov;
}
void Camera::setFovDegrees(float fovDegrees) {
    this->fov = fovDegrees * M_PI / 180;
}
float Camera::getFocusDistance() const {
    return focusDistance;
}
void Camera::setFocusDistance(float focusDistance) {
    this->focusDistance = focusDistance;
}
float Camera::getDefocusAngle() const {
    return defocusAngle;
}
void Camera::setDefocusAngle(float defocusAngle) {
    this->defocusAngle = defocusAngle;
}
void Camera::setDefocusAngleDegrees(float defocusAngleDegrees) {
    this->defocusAngle = degreesToRadians(defocusAngleDegrees);
}