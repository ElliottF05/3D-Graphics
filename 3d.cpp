#include "3d.h"
#include "2d.h"
#include <iostream>
#include <vector>

using namespace _3d;

void _3d::project(Vec3& a, Vec3& b) {
    std::cout << "in project()" << "\n";
    // make b be the OUT OF VIEW one
    // so, a is the IN VIEW one
    if (a.z < 0) {
        Vec3 temp = a;
        a = b;
        b = temp;
    }


    // _2d::Vec2 v = b.toScreenCoords(cam, window);
    // _2d::drawPoint(window, v);

    // std::cout << a.toString() + "\n";

    float dx = a.x - b.x;
    float dy = a.y - b.y;

    float distToX, distToY;

    if (dx < 0) {
        distToX = -1 - b.x;
    } else {
        distToX = 1 - b.x;
    }

    if (dy < 0) {
        distToY = -1 - b.y;
    } else {
        distToY = 1 - b.y;
    }

    // std::cout << "distToX: " << distToX << ", distToY: " << distToY << "\n";
    // std::cout << "dx: " << dx << ", dy: " << dy << "\n";

    // Normally, this would have a < symbol to find whichever border is closer.
    // To find whichever border is farther, use a > symbol
    if (abs(distToX * dy) > abs(distToY * dx)) { // distToX / dx < distToY / dy
        // x is closer
        b.y += distToX / dx * dy;
        b.x += distToX;
        std::cout << "x is chosen" << "\n";
    } else {
        // y is closer
        b.x += distToY / dy * dx;
        b.y += distToY;
        std::cout << "x is chosen" << "\n";
    }

    // std::cout << p2copy.toString() + "\n";

}

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

void Vec3::toPlaneCoords(const Camera& cam) {
    this->subtract(cam.pos);
    this->rotate(-cam.thetaZ, -cam.thetaY);
    std::cout << this->toString() << ",    ";

    float rotatedX = this->x;

    this->x = this->y / rotatedX;
    this->y = this->z / rotatedX;

    if (rotatedX > 0) {
        this->z = 1;
    } else {
        this->z = -1;
    }

}

_2d::Vec2 Vec3::toScreenCoords(const Camera& cam, sf::RenderWindow& window) {
    float maxPlaneCoordValue = tan(0.5 * cam.fov_rad);

    float screenX = (0.5 * window.getSize().x) * (1 - this->x / maxPlaneCoordValue);
    float screenY = 0.5 * window.getSize().y - this->y / maxPlaneCoordValue * 0.5 * window.getSize().x;

    return _2d::Vec2(screenX, screenY, this->z > 0);
}

void Vec3::draw(const Camera& cam, sf::RenderWindow& window) {
    Vec3 copy = *this;
    copy.toPlaneCoords(cam);
    _2d::Vec2 v = copy.toScreenCoords(cam, window);
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
    std::cout << "\n";
    Vec3 p1copy = p1;
    Vec3 p2copy = p2;

    p1copy.toPlaneCoords(cam);
    p2copy.toPlaneCoords(cam);

    std::cout << p1copy.toString() + ", " + p2copy.toString() << "\n";

    if (p1copy.z < 0 && p2copy.z < 0) {
        return;
    }

    if (p1copy.z < 0 || p2copy.z < 0) {
        _3d::project(p1copy, p2copy);
    }
    
    // _2d::Vec2 v1 = p1copy.toScreenCoords(cam, window);
    // _2d::Vec2 v2 = p2copy.toScreenCoords(cam, window);
    //
    // if (!v1.inFront && !v2.inFront) {
    //     return;
    // } else if (!v1.inFront) {
    //     v1.x += 10 * (v2.x - v1.x);
    //     v1.y += 10 * (v2.y - v1.y);
    // } else if (!v2.inFront) {
    //     v2.x += 10 * (v1.x - v2.x);
    //     v2.y += 10 * (v1.y - v2.y);
    // }

    _2d::Vec2 v1 = p1copy.toScreenCoords(cam, window);
    _2d::Vec2 v2 = p2copy.toScreenCoords(cam, window);
    std::cout << "(x,y): (" << v1.x << "," << v1.y << "), (x,y): (" << v2.x << "," << v2.y << ")\n";
    _2d::drawLine(window, v1, v2);
}

Triangle::Triangle(Vec3& p1, Vec3& p2, Vec3& p3) {
    this->p1 = p1;
    this->p2 = p2;
    this->p3 = p3;
}

void Triangle::draw(const Camera &cam, sf::RenderWindow &window) {
    Vec3 p1copy = p1;
    Vec3 p2copy = p2;
    Vec3 p3copy = p3;

    p1copy.toPlaneCoords(cam);
    p2copy.toPlaneCoords(cam);
    p3copy.toPlaneCoords(cam);
    std::cout << "\n";
    std::cout << p1copy.toString() + ",    " + p2copy.toString() + ",    " + p3copy.toString() + "\n";

    if (p1copy.z < 0 && p2copy.z < 0 && p3copy.z < 0) {
        return;
    }

    if (p1copy.z > 0 && p2copy.z > 0 && p3copy.z > 0) {
        _2d::Vec2 v1 = p1copy.toScreenCoords(cam, window);
        _2d::Vec2 v2 = p2copy.toScreenCoords(cam, window);
        _2d::Vec2 v3 = p3copy.toScreenCoords(cam, window);
        _2d::drawTriangle(window, v1, v2, v3);
        return;
    }

    std::vector<_2d::Vec2> points = std::vector<_2d::Vec2>();
    
    if (p1copy.z > 0 != p2copy.z > 0) {
        Vec3 a = p1copy;
        Vec3 b = p2copy;
        project(a, b);
        points.push_back(a.toScreenCoords(cam, window));
        points.push_back(b.toScreenCoords(cam, window));
    }
    if (p1copy.z > 0 != p3copy.z > 0) {
        Vec3 a = p1copy;
        Vec3 b = p3copy;
        project(a, b);
        points.push_back(a.toScreenCoords(cam, window));
        points.push_back(b.toScreenCoords(cam, window));
    }
    if (p2copy.z > 0 != p3copy.z > 0) {
        Vec3 a = p2copy;
        Vec3 b = p3copy;
        project(a, b);
        points.push_back(a.toScreenCoords(cam, window));
        points.push_back(b.toScreenCoords(cam, window));
    }

    if (points.size() != 4) {
        std::cout << "error" << "\n";
    }

    for (_2d::Vec2 v : points) {
        std::cout << "(x,y):  (" << v.x << "," << v.y << "), ";
    }
    std::cout << "\n";

    _2d::drawTriangle(window, points[0], points[1], points[2]);
    _2d::drawTriangle(window, points[2], points[3], points[1]);
    

}