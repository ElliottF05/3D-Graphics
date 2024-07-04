#include "3d.h"
#include "2d.h"
#include <algorithm>
#include <cmath>
#include <iostream>
#include <vector>

using namespace _3d;

void project(Vec3& v1, Vec3& v2) {
    // make v1 be the one in front
    if (v1.z < 0) {
        Vec3 temp = v1;
        v1 = v2;
        v2 = temp;
    }

    // KISS: KEEP IT SIMPLE STUPID (can always improve later)
    Vec3 r = v1;
    r.subtract(v2); // r = v1 - v2
    r.scalarMult(100);

    v2.add(r);

    v2.z = -1;
}

Vec3::Vec3(float x, float y, float z) {
    this->x = x;
    this->y = y;
    this->z = z;
    this->depth = 0;
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

Vec3 Vec3::operator+(const Vec3& other) const {
    return Vec3(this->x + other.x, this->y + other.y, this->z + other.z);
}
Vec3 Vec3::operator-(const Vec3& other) const {
    return Vec3(this->x - other.x, this->y - other.y, this->z - other.z);
}
Vec3 Vec3::operator*(const float scalar) const {
    return Vec3(this->x * scalar, this->y * scalar, this->z * scalar);
}
Vec3 Vec3::operator/(const float scalar) const {
    float a = 1.0 / scalar;
    return Vec3(this->x * a, this->y * a, this->z * a);
}
Vec3 operator*(const float scalar, const Vec3& vec) {
    return Vec3(vec.x * scalar, vec.y * scalar, vec.z * scalar);
}
Vec3 operator/(const float scalar, const Vec3&vec) {
    float x = 1.0 / scalar;
    return Vec3(vec.x * x, vec.y * x, vec.z * x);
}

Vec3 Vec3::cross(const Vec3& other) const {
    return Vec3(
        this->y * other.z - this->z * other.y,
        this->z * other.x - this->x * other.z,
        this->x * other.y - this->y * other.x
        );
}

float Vec3::dot(const Vec3& other) const {
    return this->x * other.x + this->y * other.y + this->z * other.z;
}

float Vec3::mag() const {
    return sqrt(this->x * this->x + this->y * this->y + this->z * this->z);
}

float Vec3::angleWith(const Vec3 &other) const {
    return acos(this->dot(other) / (this->mag() * other.mag()));
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

// Note that if rotated vector lies behind camera plane, vector produced
// by this function will still give its interception with the x=1 plane
// Setting this-> = -1 is just a formality to still communicate this information
void Vec3::toPlaneCoords() {
float rotatedX = this->x;

    this->x = this->y / rotatedX;
    this->y = this->z / rotatedX;

    if (rotatedX > 0) {
        this->z = 1;
    } else {
        this->z = -1;
    }

}

void Vec3::subtractAndRotate(const Camera&cam) {
    this->subtract(cam.pos);
    this->rotate(-cam.thetaZ, -cam.thetaY);
    this->depth = this->mag();
}

void Vec3::toScreenCoords(const Camera& cam, sf::RenderWindow& window) {
    float maxPlaneCoordValue = tan(0.5 * cam.fov_rad);

    int screenX = round((0.5 * window.getSize().x) * (1 - this->x / maxPlaneCoordValue));
    int screenY = round(0.5 * window.getSize().y - this->y / maxPlaneCoordValue * 0.5 * window.getSize().x);

    this->x = screenX;
    this->y = screenY;
}

void Vec3::fullyToPlaneCoords(const Camera& cam) {
    this->subtractAndRotate(cam);
    this->toPlaneCoords();
}


void Vec3::draw(const Camera& cam, sf::RenderWindow& window) {
    Vec3 copy = *this;
    copy.fullyToPlaneCoords(cam);
    copy.toScreenCoords(cam, window);
    if (copy.z > 0) {
        _2d::drawPoint(window, copy);
    }
}


Camera::Camera(Vec3 pos, float thetaY, float thetaZ, float fov) {
    this->pos = pos;
    this->thetaY = thetaY;
    this->thetaZ = thetaZ;
    this->fov = fov;
    this->fov_rad = fov * M_PI / 180.0;
}

Camera::Camera() {
    Vec3 a;
    Camera(a, 0, 0, 90);
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
    // TODO: fully check the correctness of this section

    Vec3 v1 = p1;
    Vec3 v2 = p2;

    v1.fullyToPlaneCoords(cam);
    v2.fullyToPlaneCoords(cam);

    if (v1.z < 0 && v2.z < 0) {
        return;
    } else if (v1.z > 0 && v2.z > 0) {
        v1.toScreenCoords(cam, window);
        v2.toScreenCoords(cam, window);
    } else {
        project(v1, v2);

        v1.toScreenCoords(cam, window);
        v2.toScreenCoords(cam, window);
    }

    _2d::drawLine(window, v1, v2);

}

std::vector<Triangle*> Triangle::triangles;

Triangle::Triangle(Vec3 p1, Vec3 p2, Vec3 p3) {
    this->p1 = p1;
    this->p2 = p2;
    this->p3 = p3;

    this->center = (p1 + p2 + p3) / 3;
    this->norm = (p1 - p2).cross(p1 - p3);
    this->distanceToCam = 0;

    Triangle::triangles.push_back(this);
}

void Triangle::draw(const Camera &cam, sf::RenderWindow &window) {
    // TODO: fully check correctness of this section

    // TODO: update this for full functionality
    // Calculate color:
    Vec3 sunDirection = Vec3(1,1,1);
    Vec3 a,b;
    a = p1;
    b = p2;
    a.subtract(p3); // p1 - p3
    b.subtract(p3); // p2 - p3

    Vec3 norm = a.cross(b);
    
    // make sure norm points towards camera
    Vec3 midpoint = p1;
    midpoint.add(p2);
    midpoint.add(p3);
    midpoint.scalarMult(1.0/3);

    Vec3 toCam = cam.pos;
    toCam.subtract(midpoint);
    if (toCam.angleWith(norm) >= M_PI / 2.0) {
        norm.scalarMult(-1);
    }

    float color = 0.1;
    if (sunDirection.angleWith(norm) < M_PI / 2.0) {
        color = 0.1 + 0.9 * cos(sunDirection.angleWith(norm));
    }



    Vec3 v1, v2, v3;
    v1 = this->p1;
    v2 = this->p2;
    v3 = this->p3;

    v1.fullyToPlaneCoords(cam);
    v2.fullyToPlaneCoords(cam);
    v3.fullyToPlaneCoords(cam);

    int inViewCount = 0;
    if (v1.z > 0) {
        inViewCount++;
    }
    if (v2.z > 0) {
        inViewCount++;
    }
    if (v3.z > 0) {
        inViewCount++;
    }

    if (inViewCount == 0) {
        return;
    } else if (inViewCount == 3) {
        v1.toScreenCoords(cam, window);
        v2.toScreenCoords(cam, window);
        v3.toScreenCoords(cam, window);
        _2d::drawTriangle(window, v1, v2, v3, color);
    } else if (inViewCount == 1) {

        Vec3 inView;
        std::vector<Vec3> outOfView;
        if (v1.z > 0) {
            inView = v1;
        } else {
            outOfView.push_back(v1);
        }

        if (v2.z > 0) {
            inView = v2;
        } else {
            outOfView.push_back(v2);
        }

        if (v3.z > 0) {
            inView = v3;
        } else {
            outOfView.push_back(v3);
        }

        if (outOfView.size() != 2) {
            throw 2;
        }

        project(inView, outOfView[0]);
        project(inView, outOfView[1]);

        inView.toScreenCoords(cam, window);
        outOfView[0].toScreenCoords(cam, window);
        outOfView[1].toScreenCoords(cam, window);
        _2d::drawTriangle(window, inView, outOfView[0], outOfView[1], color);

    } else { // inViewCount == 2
        std::vector<Vec3> inView;
        Vec3 outOfView;

        if (v1.z < 0) {
            outOfView = v1;
        } else {
            inView.push_back(v1);
        }
        if (v2.z < 0) {
            outOfView = v2;
        } else {
            inView.push_back(v2);
        }
        if (v3.z < 0) {
            outOfView = v3;
        } else {
            inView.push_back(v3);
        }

        if (inView.size() != 2) {
            throw 3;
        }

        Vec3 outOfView2 = outOfView;

        project(inView[0], outOfView);
        project(inView[1], outOfView2);

        Vec3 v4;
        inView[0].toScreenCoords(cam, window);
        outOfView.toScreenCoords(cam, window);
        inView[1].toScreenCoords(cam, window);
        outOfView2.toScreenCoords(cam, window);

        _2d::drawTriangle(window, inView[0], outOfView, inView[1], color);
        _2d::drawTriangle(window, inView[1], outOfView2, outOfView, color);
    }

}

bool Triangle::compareByDistance(Triangle* t1, Triangle* t2) {
    return t1->distanceToCam > t2->distanceToCam;
}

void Triangle::drawAll(const Camera& cam, sf::RenderWindow& window) {
    for (Triangle* trianglePointer : Triangle::triangles) {
        trianglePointer->distanceToCam = (trianglePointer->center - cam.pos).mag();
    }
    //std::sort(Triangle::triangles.begin(), Triangle::triangles.end(), Triangle::compareByDistance);
    for (Triangle* trianglePointer : Triangle::triangles) {
        Triangle triangle = *trianglePointer;
        triangle.draw(cam, window);
    }
}

World::World(Camera cam, Vec3 sunDirection) {
    this->cam = cam;
    this->sunDirection = sunDirection;
}