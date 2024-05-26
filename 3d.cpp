#include "3d.h"
#include "2d.h"
#include <iostream>

using namespace _3d;

Vec3::Vec3(float x, float y, float z) {
    this->x = x;
    this->y = y;
    this->z = z;
}

Vec3::Vec3() {
    this->x = 0;
    this->y = 0;
    this->z = 0;
};

Vec3::Vec3(const Vec3& other) {
    this->x = other.x;
    this->y = other.y;
    this->z = other.z;
}

void Vec3::add(const Vec3& other) {
    this->x += other.x;
    this->y += other.y;
    this->z += other.z;
};

void Vec3::subtract(const Vec3& other) {
    this->x -= other.x;
    this->y -= other.y;
    this->z -= other.z;
}

void Vec3::scalarMult(float k) {
    this->x *= k;
    this->y *= k;
    this->z *= k;
}

void Vec3::rotateZ(float thetaZ) {
    Vec3 orig = Vec3(*this);

    this->x = orig.x * cos(thetaZ) - orig.y * sin(thetaZ);
    this->y = orig.x * sin(thetaZ) + orig.y * cos(thetaZ);
}

void Vec3::rotateY(float thetaY) {
    Vec3 orig = Vec3(*this);

    this->x = orig.x * cos(thetaY) - orig.z * sin(thetaY);
    this->z = orig.x * sin(thetaY) + orig.z * cos(thetaY);
}

void Vec3::rotate(float thetaZ, float thetaY) {
    rotateZ(thetaZ);
    rotateY(thetaY);
}

std::string Vec3::toString() {
    return std::to_string(this->x) + ", " + std::to_string(this->y) + ", " + std::to_string(this->z);
}

Vec3 Vec3::toPlaneCoords(const Camera& cam) {
    Vec3 rotated = Vec3(*this);

    rotated.subtract(cam.pos);
    rotated.rotate(-cam.thetaZ, -cam.thetaY);

    return Vec3(rotated.y / rotated.x, rotated.z / rotated.x, (rotated.x) < 0 ? -1 : 1);
}

_2d::Vec2 Vec3::toScreenCoords(const Camera& cam, sf::RenderWindow& window) {
    Vec3 planeCoords = toPlaneCoords(cam);
    float maxPlaneCoordValue = tan(0.5 * cam.fov_rad);

    float screenX = (0.5 * window.getSize().x) * (1 - planeCoords.x / maxPlaneCoordValue);
    float screenY = 0.5 * window.getSize().y - planeCoords.y / maxPlaneCoordValue * 0.5 * window.getSize().x;

    return _2d::Vec2(screenX, screenY, planeCoords.z > 0);
}

void Vec3::draw(const Camera& cam, sf::RenderWindow& window) {
    _2d::Vec2 v = toScreenCoords(cam, window);
    if (v.inFront) {
        v.draw(window);
    }
}


Camera::Camera(Vec3 pos, float thetaY, float thetaZ, float fov) {
    this->pos = pos;
    this->thetaY = thetaY;
    this->thetaZ = thetaZ;
    this->fov = fov;
    this->fov_rad = fov * M_PI / 180.0;
}

Vec3 Camera::getUnitFloorVector() {;
    Vec3 v = Vec3(cos(thetaZ), sin(thetaZ), 0);
    return v;
}

void Camera::setThetaY(float angle) {
    if (angle > M_PI / 2) {
        angle = M_PI / 2;
    }
    if (angle < -M_PI / 2) {
        angle = -M_PI / 2;
    }
    this->thetaY = angle;
}

void Camera::setThetaZ(float angle) {
    this->thetaZ = angle;
}



Line::Line(Vec3& p1, Vec3& p2) {
    this->p1 = p1;
    this->p2 = p2;
}

void Line::draw(const Camera& cam, sf::RenderWindow& window) {
    _2d::Vec2 v1 = p1.toScreenCoords(cam, window);
    _2d::Vec2 v2 = p2.toScreenCoords(cam, window);

    if (!v1.inFront && !v2.inFront) {
        return;
    } else if (!v1.inFront) {
        v1.x += 10 * (v2.x - v1.x);
        v1.y += 10 * (v2.y - v1.y);
    } else if (!v2.inFront) {
        v2.x += 10 * (v1.x - v2.x);
        v2.y += 10 * (v1.y - v2.y);
    }

    _2d::drawLine(window, v1, v2);
}

Triangle::Triangle(Vec3& p1, Vec3& p2, Vec3& p3) {
    this->p1 = p1;
    this->p2 = p2;
    this->p3 = p3;
}

void Triangle::draw(const Camera &cam, sf::RenderWindow &window) {
    _2d::Vec2 v1 = p1.toScreenCoords(cam, window);
    _2d::Vec2 v2 = p2.toScreenCoords(cam, window);
    _2d::Vec2 v3 = p3.toScreenCoords(cam, window);


}