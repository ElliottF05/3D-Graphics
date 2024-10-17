#include "graphics.h"
#include "threads.h"
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <memory>
#include <vector>
#include <mutex>

using namespace graphics;

//-----------------------------------------------------------------------------------
// IMPLEMENTATION OF "Vec3"

// CONSTRUCTORS
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

// OPERATIONS
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
Vec3& Vec3::operator+=(const Vec3& vec) {
    this->x += vec.x;
    this->y += vec.y;
    this->z += vec.z;
    return *this;
}
Vec3& Vec3::operator-=(const Vec3& vec) {
    this->x -= vec.x;
    this->y -= vec.y;
    this->z -= vec.z;
    return *this;
}
Vec3& Vec3::operator*=(const float scalar) {
    this->x *= scalar;
    this->y *= scalar;
    this->z *= scalar;
    return *this;
}
Vec3& Vec3::operator/=(const float scalar) {
    this->x /= scalar;
    this->y /= scalar;
    this->z /= scalar;
    return *this;
}
// Operators where Vec3 is right-hand-side
Vec3 graphics::operator*(const float scalar, const Vec3& vec) {
    return Vec3(vec.x * scalar, vec.y * scalar, vec.z * scalar);
}
Vec3 graphics::operator/(const float scalar, const Vec3&vec) {
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
void Vec3::normalize() {
    *this /= mag();
}
float Vec3::angleWith(const Vec3 &other) const {
    return acos(this->dot(other) / (this->mag() * other.mag()));
}

// ROTATION
void Vec3::rotateZ(float thetaZ) {
    Vec3 orig = *this;

    this->x = orig.x * cos(thetaZ) - orig.y * sin(thetaZ);
    this->y = orig.x * sin(thetaZ) + orig.y * cos(thetaZ);
}
void Vec3::rotateY(float thetaY) {
    Vec3 orig = *this;

    this->x = orig.x * cos(thetaY) - orig.z * sin(thetaY);
    this->z = orig.x * sin(thetaY) + orig.z * cos(thetaY);
}
void Vec3::rotate(float thetaZ, float thetaY) {
    rotateZ(thetaZ);
    rotateY(thetaY);
}
void Vec3::rotateZKnownTrig(float sinthetaZ, float costhetaZ) {
    Vec3 orig = *this;

    this->x = orig.x * costhetaZ - orig.y * sinthetaZ;
    this->y = orig.x * sinthetaZ + orig.y * costhetaZ;
}
void Vec3::rotateYKnownTrig(float sinthetaY, float costhetaY) {
    Vec3 orig = *this;

    this->x = orig.x * costhetaY - orig.z * sinthetaY;
    this->z = orig.x * sinthetaY + orig.z * costhetaY;
}
void Vec3::rotateKnownTrig(float sinthetaZ, float costhetaZ, float sinthetaY, float costhetaY) {
    rotateZKnownTrig(sinthetaZ, costhetaZ);
    rotateYKnownTrig(sinthetaY, costhetaY);
}

// TO STRING
std::string Vec3::toString() {
    return std::to_string(this->x) + ", " + std::to_string(this->y) + ", " + std::to_string(this->z);
}



//-----------------------------------------------------------------------------------
// IMPLEMENTATION OF "Point"

// CONSTRUCTORS
Point::Point(Vec3 absolutePos) {
    this->absolutePos = absolutePos;
}
Point::Point(float x, float y, float z) {
    absolutePos = Vec3(x, y, z);
}
Point::Point() {}

// METHODS
void Point::calculateCameraPos(const Camera &cam) {
    cameraPos = absolutePos - cam.pos;
    // cameraPos.rotate(-cam.thetaZ, -cam.thetaY);
    cameraPos.rotateKnownTrig(-cam.sinthetaZ, cam.costhetaZ, -cam.sinthetaY, cam.costhetaY);
    distToCamera = cameraPos.mag();
}
void Point::calculateProjectedPos() {
    // NOTE: x, y, and z now carry different meanings. x = horizontal pos, y = vertical pos
    projectedPos.y = cameraPos.z / cameraPos.x;
    projectedPos.x = cameraPos.y / cameraPos.x;

    if (cameraPos.x > 0) {
        projectedPos.z = 1;
    } else {
        projectedPos.z = -1;
    }
}
void Point::calculateScreenPos(const Camera& cam, const Window &window) {
    screenPos.x = (0.5 * window.width) * (1 - projectedPos.x * cam.maxPlaneCoordInv) - 0.5;
    screenPos.y = 0.5 * (window.height - projectedPos.y * cam.maxPlaneCoordInv * window.width) - 0.5;
}
void Point::calculateScreenPos(const Camera& cam, const int width, const int height) {
    screenPos.x = (0.5 * width) * (1 - projectedPos.x * cam.maxPlaneCoordInv) - 0.5;
    screenPos.y = 0.5 * (height - projectedPos.y * cam.maxPlaneCoordInv * width) - 0.5;
}
void Point::calculateAll(const Camera& cam, const Window& window) {
    calculateCameraPos(cam);
    calculateProjectedPos();
    calculateScreenPos(cam, window);
}
std::string Point::toString() {
    return "absolutePos: " + absolutePos.toString() + ", cameraPos: " + cameraPos.toString() + ", projectedPos: " + projectedPos.toString() + ", screenPos: " + screenPos.toString();
}


//-----------------------------------------------------------------------------------
// IMPLEMENTATION OF "Line"

// CONSTRUCTORS
Line::Line(Point p1, Point p2) {
    this->p1 = p1;
    this->p2 = p2;
}
Line::Line(Vec3 p1, Vec3 p2) : p1(p1), p2(p2) {}
Line::Line() {}

// METHODS
void Line::draw(const Camera& cam, Window& window) {
    p1.calculateCameraPos(cam);
    p1.calculateProjectedPos();

    p2.calculateCameraPos(cam);
    p2.calculateProjectedPos();

    // check if both points are behind the camera
    if (p1.projectedPos.z < 0 && p2.projectedPos.z < 0) {
        return;
    } else if (p1.projectedPos.z > 0 && p2.projectedPos.z > 0) {
        p1.calculateScreenPos(cam, window);
        p2.calculateScreenPos(cam, window);
        window.drawLine(*this);
    } else {
        if (p1.projectedPos.z < 0) { // p1 is offscreen
            p1.projectedPos += 100 * (p2.projectedPos - p1.projectedPos);
            p1.projectedPos.z = 1;
        } else { // p2 is offscreen
            p2.projectedPos += 100 * (p1.projectedPos - p2.projectedPos);
            p1.projectedPos.z = 2;
        }
        p1.calculateScreenPos(cam, window);
        p2.calculateScreenPos(cam, window);
        window.drawLine(*this);
    }
}


//-----------------------------------------------------------------------------------
// IMPLEMENTATION OF "Triangle"

// CONSTRUCTOR
// NOTE: When points are given in clockwise order, the normal vector points towards the camera
std::vector<Triangle> Triangle::triangles; // defining the static variable, TODO: check if this is right
Triangle::Triangle(Vec3 p1, Vec3 p2, Vec3 p3, int r, int g, int b) : p1(p1), p2(p2), p3(p3), r(r), g(g), b(b) {
    this->absoluteNormal = (p3 - p1).cross(p2 - p1);
    this->absoluteNormal.normalize();
}
Triangle::Triangle(Point p1, Point p2, Point p3) {
    this->p1 = p1;
    this->p2 = p2;
    this->p3 = p3;
    this->absoluteNormal = (p3.absolutePos - p1.absolutePos).cross(p2.absolutePos - p1.absolutePos);
    absoluteNormal.normalize();
    this->r = std::rand() % 256;
    this->g = std::rand() % 256;
    this->b = std::rand() % 256;
}
Triangle::Triangle(Vec3 p1, Vec3 p2, Vec3 p3) : p1(p1), p2(p2), p3(p3) {
    absoluteNormal = (p3 - p1).cross(p2 - p1);
    absoluteNormal.normalize();
    this->r = std::rand() % 256;
    this->g = std::rand() % 256;
    this->b = std::rand() % 256;
}
Triangle::Triangle() {}

// METHODS
void Triangle::draw(Camera& cam, Window& window, const Object3D& object) {
    Vec3 toCam = cam.pos - p1.absolutePos;
    if (absoluteNormal.dot(toCam) < 0) {
        return;
    }
    p1.calculateCameraPos(cam);
    p2.calculateCameraPos(cam);
    p3.calculateCameraPos(cam);

    cameraNormal = (p2.cameraPos - p1.cameraPos).cross(p3.cameraPos - p1.cameraPos);
    cameraNormal.normalize();

    p1.calculateProjectedPos();
    p2.calculateProjectedPos();
    p3.calculateProjectedPos();

    std::array<Point*, 3> front;
    int frontSize = 0;
    std::array<Point*, 3> behind;
    int behindSize = 0;

    if (p1.projectedPos.z > 0) {
        front[frontSize] = &p1;
        ++frontSize;
    } else {
        behind[behindSize] = &p1;
        ++behindSize;
    }
    if (p2.projectedPos.z > 0) {
        front[frontSize] = &p2;
        ++frontSize;
    } else {
        behind[behindSize] = &p2;
        ++behindSize;
    }
    if (p3.projectedPos.z > 0) {
        front[frontSize] = &p3;
        ++frontSize;
    } else {
        behind[behindSize] = &p3;
        ++behindSize;
    }

    if (frontSize == 0) {
        return;
    } else if (frontSize == 1) {
        behind[0]->projectedPos += 100 * (front[0]->projectedPos - behind[0]->projectedPos);
        behind[0]->projectedPos.z = 1;
        behind[1]->projectedPos += 100 * (front[0]->projectedPos - behind[1]->projectedPos);
        behind[1]->projectedPos.z = 1;

        front[0]->calculateScreenPos(cam, window);
        behind[0]->calculateScreenPos(cam, window);
        behind[1]->calculateScreenPos(cam, window);

        window.drawTriangle(*this, object, cam);
    } else if (frontSize == 2) {
        front[0]->calculateScreenPos(cam, window);
        front[1]->calculateScreenPos(cam, window);

        Point behind2 = *behind[0];
        behind[0]->projectedPos += 100 * (front[0]->projectedPos - behind[0]->projectedPos);
        behind2.projectedPos += 100 * (front[1]->projectedPos - behind2.projectedPos);

        behind[0]->calculateScreenPos(cam, window);
        behind2.calculateScreenPos(cam, window);

        // needed to preserve the 3 original cameraPos values to be used in plane-calculation for depth buffer
        behind2.cameraPos = front[0]->cameraPos;

        std::shared_ptr<Triangle> t = std::shared_ptr<Triangle>(new Triangle(*front[1], *behind[0], behind2));
        t->absoluteNormal = absoluteNormal;
        t->cameraNormal = cameraNormal;
        t->r = r;
        t->g = g;
        t->b = b;

        window.drawTriangle(*this, object, cam);
        window.drawTriangle(t, object, cam);
    } else {
        front[0]->calculateScreenPos(cam, window);
        front[1]->calculateScreenPos(cam, window);
        front[2]->calculateScreenPos(cam, window);

        window.drawTriangle(*this, object, cam);
    }
}
void Triangle::drawVerticalScreenLine(Camera &cam, Window &window, const Triangle &triangle, const Object3D& object, int x, float y1, float y2, float d1) {
    int bottom = round(y1);
    int top = round(y2);
    utils::sortAndClamp(bottom, top, window.height - 1);
    float cameraY = cam.getCameraYFromPixelFast(x, window.widthInv);
    float depth;
    for (int y = bottom; y <= top; y++) {
        // calculate depth
        float cameraZ = cam.getCameraZFromPixelFast(y, window.heightInv);
        float cameraX = 1;
        float denom = triangle.cameraNormal.x * cameraX + triangle.cameraNormal.y * cameraY + triangle.cameraNormal.z * cameraZ;
        float cameraVecLength = sqrt(cameraX * cameraX + cameraY * cameraY + cameraZ * cameraZ);
        depth = (d1 / denom) * cameraVecLength;
        depth = std::max(depth, 0.0f);

        if (depth < window.zBuffer.getDepth(x, y)) {
            window.zBuffer.setDepth(x, y, depth);

            if (x == window.width * 0.5 && y == window.height * 0.5) {
                cam.lookingAtTriangle = &triangle;
                cam.lookingAtObject = &object;
            }

            Vec3 vec(cameraX, cameraY, cameraZ);
            vec /= cameraVecLength;
            vec *= depth;
            // vec.rotateY(cam.thetaY);
            vec.rotateYKnownTrig(cam.sinthetaY, cam.costhetaY);
            // vec.rotateZ(cam.thetaZ);
            vec.rotateZKnownTrig(cam.sinthetaZ, cam.costhetaZ);
            vec += cam.pos;

            Vec3 vecToLight = Light::lights[0].cam.pos - vec;
            float vecToLightMagInv = 1.0 / vecToLight.mag();
            vecToLight *= vecToLightMagInv;
            float shadowMapLightingAmount = Light::lights[0].amountLit(vec, vecToLightMagInv);
            float angleLighting = vecToLight.dot(triangle.absoluteNormal);

            float multiplier;
            if (angleLighting > 0) {
                multiplier = 0.2 + 0.8 * shadowMapLightingAmount * angleLighting;
            } else {
                multiplier = 0.2 + 0.05 * angleLighting;
            }
            window.pixelArray.setPixel(x, y, multiplier * triangle.r, multiplier * triangle.g, multiplier * triangle.b);
        }
    }
}
void Triangle::drawVerticalScreenLine(Camera& cam, Window& window, const std::shared_ptr<Triangle> triangle, const Object3D& object, int x, float y1, float y2, float d1) {
    int bottom = round(y1);
    int top = round(y2);
    utils::sortAndClamp(bottom, top, window.height - 1);
    float cameraY = cam.getCameraYFromPixelFast(x, window.widthInv);
    float depth;
    for (int y = bottom; y <= top; y++) {
        // calculate depth
        float cameraZ = cam.getCameraZFromPixelFast(y, window.heightInv);
        float cameraX = 1;
        float denom = triangle->cameraNormal.x * cameraX + triangle->cameraNormal.y * cameraY + triangle->cameraNormal.z * cameraZ;
        float cameraVecLength = sqrt(cameraX * cameraX + cameraY * cameraY + cameraZ * cameraZ);
        depth = (d1 / denom) * cameraVecLength;
        depth = std::max(depth, 0.0f);

        if (depth < window.zBuffer.getDepth(x, y)) {
            window.zBuffer.setDepth(x, y, depth);

            if (x == window.width * 0.5 && y == window.height * 0.5) {
                cam.lookingAtTriangle = triangle.get();
                cam.lookingAtObject = &object;
            }

            Vec3 vec(cameraX, cameraY, cameraZ);
            vec /= cameraVecLength;
            vec *= depth;
            // vec.rotateY(cam.thetaY);
            vec.rotateYKnownTrig(cam.sinthetaY, cam.costhetaY);
            // vec.rotateZ(cam.thetaZ);
            vec.rotateZKnownTrig(cam.sinthetaZ, cam.costhetaZ);
            vec += cam.pos;

            Vec3 vecToLight = Light::lights[0].cam.pos - vec;
            float vecToLightMagInv = 1.0 / vecToLight.mag();
            vecToLight *= vecToLightMagInv;
            float shadowMapLightingAmount = Light::lights[0].amountLit(vec, vecToLightMagInv);
            float angleLighting = vecToLight.dot(triangle->absoluteNormal);

            float multiplier;
            if (angleLighting > 0) {
                multiplier = 0.2 + 0.8 * shadowMapLightingAmount * angleLighting;
            } else {
                multiplier = 0.2 + 0.05 * angleLighting;
            }
            window.pixelArray.setPixel(x, y, multiplier * triangle->r, multiplier * triangle->g, multiplier * triangle->b);
        }
    }
}


//-----------------------------------------------------------------------------------
// IMPLEMENTATION OF "Object3D"

// STATIC VARIABLE
std::vector<Object3D> Object3D::objects;
int Object3D::objectCounter = 0;

// CONSTRUCTORS
Object3D::Object3D(std::vector<Triangle> triangles, bool isDeletable) {
    this->triangles = triangles;
    this->isDeletable = isDeletable;
    id = ++objectCounter;
}
Object3D::Object3D(std::vector<Triangle> triangles) : Object3D(triangles, true) {}
Object3D::Object3D() : Object3D(std::vector<Triangle>(), true) {}

// METHODS
bool Object3D::operator==(const Object3D& other) const {
    return this->id == other.id;
}
void Object3D::drawMultithreaded(Camera& cam, Window& window) {
    for (Triangle& triangle : triangles) {
        threads::threadPool.addTask([&triangle, &cam, &window, this] {
            triangle.draw(cam, window, *this);
        });
    }
}

// STATIC METHODS
void Object3D::removeObject(const Object3D& object) {
    if (!object.isDeletable) {
        return;
    }
    for (int i = 0; i < objects.size(); i++) {
        if (objects[i] == object) {
            objects.erase(objects.begin() + i);
            return;
        }
    }
}

// Making new objects
Object3D Object3D::buildCube(Vec3 center, float sideLength, int red, int green, int blue) {
    Object3D cube;
    float halfSide = sideLength / 2;
    // start at "bottom right" corner and go counter clockwise
    Vec3 a(center.x - halfSide, center.y + halfSide, center.z - halfSide);
    Vec3 b(center.x + halfSide, center.y + halfSide, center.z - halfSide);
    Vec3 c(center.x + halfSide, center.y - halfSide, center.z - halfSide);
    Vec3 d(center.x - halfSide, center.y - halfSide, center.z - halfSide);

    Vec3 e(center.x - halfSide, center.y + halfSide, center.z + halfSide);
    Vec3 f(center.x + halfSide, center.y + halfSide, center.z + halfSide);
    Vec3 g(center.x + halfSide, center.y - halfSide, center.z + halfSide);
    Vec3 h(center.x - halfSide, center.y - halfSide, center.z + halfSide);

    // front face
    cube.triangles.push_back(Triangle(a, e, h, red, green, blue));
    cube.triangles.push_back(Triangle(h, d, a, red, green, blue));

    // back face
    cube.triangles.push_back(Triangle(c, g, f, red, green, blue));
    cube.triangles.push_back(Triangle(f, b, c, red, green, blue));

    // left face
    cube.triangles.push_back(Triangle(d, h, g, red, green, blue));
    cube.triangles.push_back(Triangle(g, c, d, red, green, blue));

    // right face
    cube.triangles.push_back(Triangle(b, f, e, red, green, blue));
    cube.triangles.push_back(Triangle(e, a, b, red, green, blue));

    // top face
    cube.triangles.push_back(Triangle(e, f, g, red, green, blue));
    cube.triangles.push_back(Triangle(g, h, e, red, green, blue));

    // bottom face
    cube.triangles.push_back(Triangle(c, b, a, red, green, blue));
    cube.triangles.push_back(Triangle(a, d ,c, red, green, blue));

    return cube;
}
Object3D Object3D::buildCube(Vec3 center, float sideLength) {
    return buildCube(center, sideLength, 255, 255, 255);
}
Object3D Object3D::buildSphere(Vec3 center, float radius, int iterations, int r, int g, int b) {
    Object3D sphere;
    std::vector<graphics::Vec3> prev(iterations);
    std::vector<graphics::Vec3> curr(iterations);
    bool onFirst = true;
    for (float thetaY = -M_PI / 2.0; thetaY <= M_PI / 2.0; thetaY += M_PI / iterations) {
        prev = curr;
        curr.clear();
        for (float thetaZ = 0; thetaZ <= 2 * M_PI; thetaZ += 2 * M_PI / iterations) {
            graphics::Vec3 v(std::cos(thetaY) * std::cos(thetaZ), std::cos(thetaY) * std::sin(thetaZ), std::sin(thetaY));
            v *= radius / 2;
            v += center;
            curr.push_back(v);
        }
        if (onFirst) {
            onFirst = false;
            continue;
        }
        for (int i = 0; i < prev.size(); i++) {
            graphics::Triangle t1(prev[i], curr[i], curr[(i + 1) % iterations], r, g, b);
            graphics::Triangle t2(prev[(i + 1) % iterations], prev[i], curr[(i + 1) % iterations], r, g, b);
            t1.r = 255;
            t1.g = 255;
            t1.b = 255;
            t2.r = 255;
            t2.g = 255;
            t2.b = 255;
            sphere.triangles.push_back(t1);
            sphere.triangles.push_back(t2);
        }
    }
    return sphere;
}
Object3D Object3D::buildSphere(Vec3 center, float radius, int iterations) {
    return buildSphere(center, radius, iterations, 255, 255, 255);
}


//-----------------------------------------------------------------------------------
// IMPLEMENTATION OF "Camera"

// CONSTRUCTORS
Camera::Camera(Vec3 pos, float thetaZ, float thetaY, float fov) {
    this->pos = pos;
    this->thetaZ = thetaZ;
    this->thetaY = thetaY;
    this->fov = fov;
    this->fov_rad = fov * M_PI / 180;
    this->maxPlaneCoord = tan(fov_rad / 2);
    Vec3 a(1,0,0);
    a.rotate(thetaZ, thetaY);
    this->direction = a;
    Vec3 b(1,0,0);
    b.rotate(thetaZ, 0);
    this->floorDirection = b;
    this->sinthetaY = sin(thetaY);
    this->sinthetaZ = sin(thetaZ);
    this->costhetaY = cos(thetaY);
    this->costhetaZ = cos(thetaZ);
    this->maxPlaneCoordInv = 1 / this->maxPlaneCoord;
}
Camera::Camera() : Camera(Vec3(0,0,0), 0, 0, 90) {
}

// METHODS
void Camera::moveRelative(float forward, float sideward, float upward) {
    Vec3 sideDirection = floorDirection;
    sideDirection.rotate(-M_PI / 2, 0);
    pos += floorDirection * forward + sideDirection * sideward;
    pos.z += upward;
}
void Camera::rotate(float thetaZ, float thetaY) {
    this->thetaZ += thetaZ;
    this->thetaY += thetaY;
    this->thetaY = std::max(this->thetaY, (float) -M_PI / 2);
    this->thetaY = std::min(this->thetaY, (float) M_PI / 2);
    direction.x = 1;
    direction.y = 0;
    direction.z = 0;
    direction.rotateY(this->thetaY);
    direction.rotateZ(this->thetaZ);
    // direction.rotate(this->thetaZ, this->thetaY);
    floorDirection.rotate(thetaZ, 0);
    this->sinthetaY = sin(this->thetaY);
    this->sinthetaZ = sin(this->thetaZ);
    this->costhetaY = cos(this->thetaY);
    this->costhetaZ = cos(this->thetaZ);
}
float Camera::getCameraYFromPixel(int x, int width) const {
    return - maxPlaneCoord * (x - (0.5 * width) + 0.5) / (0.5 * width);
}
float Camera::getCameraZFromPixel(int y, int height) const {
    return - maxPlaneCoord * (y - (0.5 * height) + 0.5) / (0.5 * height);
}
float Camera::getCameraYFromPixelFast(int x, float widthInv) const {
    return - maxPlaneCoord * ((2 * x + 1.0) * widthInv - 1);
}
float Camera::getCameraZFromPixelFast(int y, float heightInv) const {
    return - maxPlaneCoord * ((2 * y + 1.0) * heightInv - 1);
}
Vec3 Camera::getCenterOfViewPosition(Window& window) const {
    float depth = window.zBuffer.getDepth(window.width * 0.5, window.height * 0.5);
    return pos + direction * depth;
}
Vec3 Camera::getPositionOfNewObject(Window& window) const {
    Vec3 viewCenter = getCenterOfViewPosition(window);
    viewCenter += 0.5 * lookingAtTriangle->absoluteNormal;
    viewCenter.x = round(viewCenter.x + 0.5) - 0.5;
    viewCenter.y = round(viewCenter.y + 0.5) - 0.5;
    viewCenter.z = round(viewCenter.z + 0.5) - 0.5;
    return viewCenter;
}


//-----------------------------------------------------------------------------------
// IMPLEMENTATION OF "PixelArray"
PixelArrayData::PixelArrayData(int r, int g, int b) {
    this->r = r;
    this->g = g;
    this->b = b;
}
PixelArrayData::PixelArrayData(int color) {
    this->r = color;
    this->g = color;
    this->b = color;
}
PixelArrayData::PixelArrayData(const PixelArrayData& other) {
    this->r = other.r;
    this->g = other.g;
    this->b = other.b;
}
PixelArrayData::PixelArrayData() {
    this->r = 0;
    this->g = 0;
    this->b = 0;
}

// CONSTRUCTOR
PixelArray::PixelArray(int width, int height) {
    this->width = width;
    this->height = height;
    data = std::vector<PixelArrayData>(width * height);
}

// METHODS
int PixelArray::getIndex(int x, int y) {
    if (x < 0 || x >= width || y < 0 || y >= height) {
        std::cout << "PixelArray::getIndex() failed, pixel coordinates out of bounds. INPUTS: x = " << x << 
        ", y = " << y << std::endl; 
        throw "pixel coordinates out of bounds";
    }
    return (width * y) + x;
}
void PixelArray::setPixel(int x, int y, int color) {
    if (color < 0 || color > 255) {
        std::cout << "PixelArray::setPixel() failed, color value out of bounds. INPUTS: color = " << color << std::endl;
        throw "color value out of bounds";
    }
    int index = this->getIndex(x, y);
    {
        std::lock_guard<std::mutex> lock(data[index].mutex);
        data[index].r = color;
        data[index].g = color;
        data[index].b = color;
    }
}
void PixelArray::setPixel(int x, int y, int r, int g, int b) {
    if (r < 0 || g < 0 || b < 0 || r > 255 || g > 255 || b > 255) {
        std::cout << "PixelArray::setPixel() failed, color value out of bounds. INPUTS: r, g, b = " << r << ", " << g << ", " << b << std::endl;
        throw "color value out of bounds";
    }
    int index = this->getIndex(x, y);
    {
        std::lock_guard<std::mutex> lock(data[index].mutex);
        data[index].r = r;
        data[index].g = g;
        data[index].b = b;
    }
}
void PixelArray::clear() {
    for (int i = 0; i < data.size(); i += width) {
        threads::threadPool.addTask([i, this] {
            for (int j = i; j < i + width; j++) {
                data[j].r = 0;
                data[j].g = 0;
                data[j].b = 0;   
            }
        });
    }

    for (int i = 0; i < data.size(); i++) {
        data[i].r = 0;
        data[i].g = 0;
        data[i].b = 0;
    }
}


//-----------------------------------------------------------------------------------
// IMPLEMENTATION OF "ZBuffer"
ZBufferData::ZBufferData(float depth) {
    this->depth = depth;
}
ZBufferData::ZBufferData(const ZBufferData& other) {
    this->depth = other.depth;
}
ZBufferData::ZBufferData() {
    this->depth = 99999;
}

// CONSTRUCTOR
ZBuffer::ZBuffer(int width, int height) {
    this->width = width;
    this->height = height;
    data = std::vector<ZBufferData>(width * height);
}

// METHODS
int ZBuffer::getIndex(int x, int y) {
    if (x < 0 || x >= width || y < 0 || y >= height) {
        std::cout << "ZBuffer::getIndex() failed, pixel coordinates out of bounds. INPUTS: x = " << x << 
        ", y = " << y << std::endl; 
        throw "pixel coordinates out of bounds";
    }
    return ((this->width * y) + x);
}
void ZBuffer::setDepth(int x, int y, float depth) {
    if (depth < 0) {
        std::cout << "ZBuffer::setDepth() failed, depth value out of bounds. INPUTS: depth = " << depth << std::endl; 
        throw "invalid depth";
    }
    int index = getIndex(x, y);
    {
        std::lock_guard<std::mutex> lock(data[index].mutex);
        data[index].depth = depth;
    }
}
float ZBuffer::getDepth(int x, int y) {
    int index = getIndex(x, y);
    return data[index].depth;
}
void ZBuffer::clear() {
    for (int i = 0; i < data.size(); i += width) {
        threads::threadPool.addTask([i, this] {
            for (int j = i; j < i + width; j++) {
                data[j].depth = 99999;
            }
        });
    }
}


//-----------------------------------------------------------------------------------
// IMPLEMENTATION OF "Window"

// CONSTRUCTOR
Window::Window(int width, int height)
 : pixelArray(width, height), zBuffer(width, height) {
    this->width = width;
    this->height = height;
    this->widthInv = 1.0 / width;
    this->heightInv = 1.0 / height;
}

void Window::drawPoint(Point& point) {
    if (point.screenPos.x >= 0 && point.screenPos.x < width
    && point.screenPos.y >= 0 && point.screenPos.y < height) {
        pixelArray.setPixel(point.screenPos.x, point.screenPos.y, 255);
    }
}
void Window::drawLine(Line &line) {
    Point a, b;
    a = line.p1;
    b = line.p2;

    // first make a = leftmost, b = rightmost point
    if (a.screenPos.x > b.screenPos.x) {
        std::swap(a, b);
    }

    float dy = (b.screenPos.y - a.screenPos.y) / (b.screenPos.x - a.screenPos.x);
    int startVal = ceil(a.screenPos.x);
    int endVal = floor(b.screenPos.x);
    startVal = std::max(0, startVal);
    endVal = std::min(width - 1, endVal);

    if (startVal >= endVal) {
        startVal = round(a.screenPos.x);
        if (startVal < 0 || startVal >= width) {
            return;
        }
        int bottom = round(a.screenPos.y);
        int top = round(b.screenPos.y);
        if (top < bottom) {
            std::swap(top, bottom);
        }
        bottom = std::max(0, bottom);
        top = std::min(height - 1, top);
        for (int yVal = bottom; yVal <= top; yVal++) {
            pixelArray.setPixel(startVal, yVal, 255);
        }
        return;
    }

    float y = a.screenPos.y + dy * (startVal - a.screenPos.x);
    for (int x = startVal; x < endVal; x++) {
        int bottom = round(y);
        int top = round(y + dy);
        if (top < bottom) {
            std::swap(top, bottom);
        }
        bottom = std::max(0, bottom);
        top = std::min(height - 1, top);
        for (int yVal = bottom; yVal <= top; yVal++) {
            pixelArray.setPixel(x, yVal, 255);
        }
        y += dy;
    }

    int bottom = round(a.screenPos.y);
    int top = round(a.screenPos.y + (startVal - a.screenPos.x) * dy);
    if (top < bottom) {
        std::swap(top, bottom);
    }
    bottom = std::max(0, bottom);
    top = std::min(height - 1, top);
    int x = floor(a.screenPos.x);
    if (x < 0 || x >= width) {
        return;
    }
    for (int yVal = bottom; yVal <= top; yVal++) {
        pixelArray.setPixel(x, yVal, 255);
    }

    bottom = round(b.screenPos.y);
    top = round(b.screenPos.y - (b.screenPos.x - endVal) * dy);
    if (top < bottom) {
        std::swap(top, bottom);
    }
    bottom = std::max(0, bottom);
    top = std::min(height - 1, top);
    x = round(b.screenPos.x);
    if (x < 0 || x >= width) {
        return;
    }
    for (int yVal = bottom; yVal <= top; yVal++) {
        pixelArray.setPixel(x, yVal, 255);
    }
    
}
void Window::drawTriangle(Triangle &triangle, const Object3D& object, Camera& cam) {
    Point a, b, c;
    a = triangle.p1;
    b = triangle.p2;
    c = triangle.p3;

    // equation for plane
    float d1 = triangle.cameraNormal.x * a.cameraPos.x + triangle.cameraNormal.y * a.cameraPos.y + triangle.cameraNormal.z * a.cameraPos.z;

    // first make a = leftmost, b = middle, c = rightmost point
    if (a.screenPos.x > b.screenPos.x) {
        std::swap(a, b);
    }
    if (b.screenPos.x > c.screenPos.x) {
        std::swap(b, c);
    }
    if (a.screenPos.x > b.screenPos.x) {
        std::swap(a, b);
    }

    float dy_long = (c.screenPos.y - a.screenPos.y) / (c.screenPos.x - a.screenPos.x);
    float dy1 = (b.screenPos.y - a.screenPos.y) / (b.screenPos.x - a.screenPos.x);
    float dy2 = (c.screenPos.y - b.screenPos.y) / (c.screenPos.x - b.screenPos.x);

    float left = a.screenPos.x;
    float  mid = b.screenPos.x;
    float right = c.screenPos.x;
    utils::clampToRange(left, width - 1);
    utils::clampToRange(mid, width - 1);
    utils::clampToRange(right, width - 1);

    // std::cout << left << ", " << mid << ", " << right << "\n";

    float y1, y2; // y1 for shorter line segment, y2 for longer line segment
    int bottom, top;
    y1 = a.screenPos.y + dy1 * (left - a.screenPos.x);
    y2 = a.screenPos.y + dy_long * (left - a.screenPos.x);
    for (float x = left; x < mid; x++) {
        threads::threadPool.addTask([&cam, this, &triangle, &object, x, y1, y2, d1] {
            Triangle::drawVerticalScreenLine(cam, *this, triangle, object, x, y1, y2, d1);
        });
        y1 += dy1;
        y2 += dy_long;
    }

    y1 = b.screenPos.y + dy2 * (mid - b.screenPos.x);
    y2 = a.screenPos.y + dy_long * (mid - a.screenPos.x);
    for (float x = mid; x < right; x++) {
        threads::threadPool.addTask([&cam, this, &triangle, &object, x, y1, y2, d1] {
            Triangle::drawVerticalScreenLine(cam, *this, triangle, object, x, y1, y2, d1);
        });
        y1 += dy2;
        y2 += dy_long;
    }
}
void Window::drawTriangle(std::shared_ptr<Triangle> triangle, const Object3D& object, Camera& cam) {
    Point a, b, c;
    a = triangle->p1;
    b = triangle->p2;
    c = triangle->p3;

    // equation for plane
    float d1 = triangle->cameraNormal.x * a.cameraPos.x + triangle->cameraNormal.y * a.cameraPos.y + triangle->cameraNormal.z * a.cameraPos.z;

    // first make a = leftmost, b = middle, c = rightmost point
    if (a.screenPos.x > b.screenPos.x) {
        std::swap(a, b);
    }
    if (b.screenPos.x > c.screenPos.x) {
        std::swap(b, c);
    }
    if (a.screenPos.x > b.screenPos.x) {
        std::swap(a, b);
    }

    float dy_long = (c.screenPos.y - a.screenPos.y) / (c.screenPos.x - a.screenPos.x);
    float dy1 = (b.screenPos.y - a.screenPos.y) / (b.screenPos.x - a.screenPos.x);
    float dy2 = (c.screenPos.y - b.screenPos.y) / (c.screenPos.x - b.screenPos.x);

    float left = a.screenPos.x;
    float  mid = b.screenPos.x;
    float right = c.screenPos.x;
    utils::clampToRange(left, width - 1);
    utils::clampToRange(mid, width - 1);
    utils::clampToRange(right, width - 1);

    // std::cout << left << ", " << mid << ", " << right << "\n";

    float y1, y2; // y1 for shorter line segment, y2 for longer line segment
    int bottom, top;
    y1 = a.screenPos.y + dy1 * (left - a.screenPos.x);
    y2 = a.screenPos.y + dy_long * (left - a.screenPos.x);
    for (float x = left; x < mid; x++) {
        threads::threadPool.addTask([&cam, this, triangle, &object, x, y1, y2, d1] {
            Triangle::drawVerticalScreenLine(cam, *this, triangle, object, x, y1, y2, d1);
        });
        y1 += dy1;
        y2 += dy_long;
    }

    y1 = b.screenPos.y + dy2 * (mid - b.screenPos.x);
    y2 = a.screenPos.y + dy_long * (mid - a.screenPos.x);
    for (float x = mid; x < right; x++) {
        threads::threadPool.addTask([&cam, this, triangle, &object, x, y1, y2, d1] {
            Triangle::drawVerticalScreenLine(cam, *this, triangle, object, x, y1, y2, d1);
        });
        y1 += dy2;
        y2 += dy_long;
    }
}
void Window::clear() {
    pixelArray.clear();
    zBuffer.clear();
}
void Window::draw() {
    // TODO: WARNING - this is implemntation specific

    auto t1 = std::chrono::high_resolution_clock::now();
    auto t2 = std::chrono::high_resolution_clock::now();
    auto t3 = std::chrono::high_resolution_clock::now();


    auto pixelTime = std::chrono::duration_cast<std::chrono::microseconds>(t2 - t1);
    auto spriteTime = std::chrono::duration_cast<std::chrono::microseconds>(t3 - t2);
    // std::cout << "inside graphics - pixel time: " << pixelTime.count() << ", sprite time: " << spriteTime.count() << "\n";
}
void Window::getUint8Pointer(uint8_t* buffer) {
    for (int i = 0; i < pixelArray.data.size(); i += width) {
        threads::threadPool.addTask([i, this, buffer] {
            for (int j = i, k = 4 * i; j < i + width; j++, k += 4) {
                buffer[k] = pixelArray.data[j].r;
                buffer[k + 1] = pixelArray.data[j].g;
                buffer[k + 2] = pixelArray.data[j].b;
                buffer[k + 3] = 255;
            }
        });
    }
}


//-----------------------------------------------------------------------------------
// IMPLEMENTATION OF "Light"
std::vector<Light> Light::lights;

// CONSTRUCTORS
Light::Light(Vec3 pos, float thetaZ, float thetaY, float fov, float luminosity) : zBuffer(4000, 4000), cam(pos, thetaZ, thetaY, fov) {
    this->luminosity = luminosity;
    filteringRadius = 2;
    filteringAreaInv = 1.0 / ( (2 * filteringRadius + 1) * (2 * filteringRadius + 1));
}
Light::Light(Vec3 pos, float thetaZ, float thetaY, float luminosity) : Light::Light(pos, thetaZ, thetaY, atan(0.5) * 180 / M_PI, luminosity) {
}

// METHODS
void Light::getTrianglePerspectiveFromLight(Triangle triangle) {
    Vec3 toCam = cam.pos - triangle.p1.absolutePos;
    if (triangle.absoluteNormal.dot(toCam) > 0) {
        return;
    }
    triangle.p1.calculateCameraPos(cam);
    triangle.p2.calculateCameraPos(cam);
    triangle.p3.calculateCameraPos(cam);

    triangle.cameraNormal = (triangle.p2.cameraPos - triangle.p1.cameraPos).cross(triangle.p3.cameraPos - triangle.p1.cameraPos);
    triangle.cameraNormal.normalize();

    triangle.p1.calculateProjectedPos();
    triangle.p2.calculateProjectedPos();
    triangle.p3.calculateProjectedPos();

    std::array<Point*, 3> front;
    int frontSize = 0;
    std::array<Point*, 3> behind;
    int behindSize = 0;

    if (triangle.p1.projectedPos.z > 0) {
        front[frontSize] = &triangle.p1;
        ++frontSize;
    } else {
        behind[behindSize] = &triangle.p1;
        ++behindSize;
    }
    if (triangle.p2.projectedPos.z > 0) {
        front[frontSize] = &triangle.p2;
        ++frontSize;
    } else {
        behind[behindSize] = &triangle.p2;
        ++behindSize;
    }
    if (triangle.p3.projectedPos.z > 0) {
        front[frontSize] = &triangle.p3;
        ++frontSize;
    } else {
        behind[behindSize] = &triangle.p3;
        ++behindSize;
    }

    if (frontSize == 0) {
        return;
    } else if (frontSize == 1) {
        behind[0]->projectedPos += 100 * (front[0]->projectedPos - behind[0]->projectedPos);
        behind[0]->projectedPos.z = 1;
        behind[1]->projectedPos += 100 * (front[0]->projectedPos - behind[1]->projectedPos);
        behind[1]->projectedPos.z = 1;

        front[0]->calculateScreenPos(cam, zBuffer.width, zBuffer.height);
        behind[0]->calculateScreenPos(cam, zBuffer.width, zBuffer.height);
        behind[1]->calculateScreenPos(cam, zBuffer.width, zBuffer.height);

        addTriangleToZBuffer(triangle);
    } else if (frontSize == 2) {
        front[0]->calculateScreenPos(cam, zBuffer.width, zBuffer.height);
        front[1]->calculateScreenPos(cam, zBuffer.width, zBuffer.height);

        Point behind2 = *behind[0];
        behind[0]->projectedPos += 100 * (front[0]->projectedPos - behind[0]->projectedPos);
        behind2.projectedPos += 100 * (front[1]->projectedPos - behind2.projectedPos);

        behind[0]->calculateScreenPos(cam, zBuffer.width, zBuffer.height);
        behind2.calculateScreenPos(cam, zBuffer.width, zBuffer.height);

        // needed to preserve the 3 original cameraPos values to be used in plane-calculation for depth buffer
        behind2.cameraPos = front[0]->cameraPos;

        Triangle t(*front[1], *behind[0], behind2);

        addTriangleToZBuffer(triangle);
        addTriangleToZBuffer(t);
    } else {
        front[0]->calculateScreenPos(cam, zBuffer.width, zBuffer.height);
        front[1]->calculateScreenPos(cam, zBuffer.width, zBuffer.height);
        front[2]->calculateScreenPos(cam, zBuffer.width, zBuffer.height);

        addTriangleToZBuffer(triangle);
    }
}
void Light::addTriangleToZBuffer(Triangle &triangle) {
    Point a, b, c;
    a = triangle.p1;
    b = triangle.p2;
    c = triangle.p3;

    // equation for plane
    Vec3 normal = (a.cameraPos - b.cameraPos).cross(a.cameraPos - c.cameraPos);
    normal.normalize();

    // make normal point TOWARDS camera
    if (normal.x < 0) {
        normal *= -1;
    }

    float d1 = normal.x * a.cameraPos.x + normal.y * a.cameraPos.y + normal.z * a.cameraPos.z;

    // first make a = leftmost, b = middle, c = rightmost point
    if (a.screenPos.x > b.screenPos.x) {
        std::swap(a, b);
    }
    if (b.screenPos.x > c.screenPos.x) {
        std::swap(b, c);
    }
    if (a.screenPos.x > b.screenPos.x) {
        std::swap(a, b);
    }

    float dy_long = (c.screenPos.y - a.screenPos.y) / (c.screenPos.x - a.screenPos.x);
    float dy1 = (b.screenPos.y - a.screenPos.y) / (b.screenPos.x - a.screenPos.x);
    float dy2 = (c.screenPos.y - b.screenPos.y) / (c.screenPos.x - b.screenPos.x);

    float left = a.screenPos.x;
    float  mid = b.screenPos.x;
    float right = c.screenPos.x;
    utils::clampToRange(left, zBuffer.width - 1);
    utils::clampToRange(mid, zBuffer.width - 1);
    utils::clampToRange(right, zBuffer.width - 1);

    // std::cout << left << ", " << mid << ", " << right << "\n";

    float y1, y2; // y1 for shorter line segment, y2 for longer line segment
    int bottom, top;
    y1 = a.screenPos.y + dy1 * (left - a.screenPos.x);
    y2 = a.screenPos.y + dy_long * (left - a.screenPos.x);
    for (float x = left; x < mid; x++) {
        bottom = round(y1);
        top = round(y2);
        utils::sortAndClamp(bottom, top, zBuffer.height - 1);
        float cameraY = cam.getCameraYFromPixel(x, zBuffer.width);
        float depth;
        for (int y = bottom; y <= top; y++) {
            // calculate depth
            float cameraZ = cam.getCameraZFromPixel(y, zBuffer.height);
            float cameraX = 1;
            depth = (d1 / (normal.x * cameraX + normal.y * cameraY + normal.z * cameraZ)) * sqrt(cameraX * cameraX + cameraY * cameraY + cameraZ * cameraZ);
            if (depth < 0) {
                if (depth < -0.1) {
                    // std::cout << "ERROR: depth < -0.1, depth = " << depth << std::endl;
                }
                depth = 0;
            }
            if (depth < zBuffer.getDepth(x, y)) {
                float depthBias = std::max(0.05 * (1 - abs(normal.x)), 0.001);
                depthBias = 0;
                zBuffer.setDepth(x, y, depth + depthBias);
            }
        }
        y1 += dy1;
        y2 += dy_long;
    }

    y1 = b.screenPos.y + dy2 * (mid - b.screenPos.x);
    y2 = a.screenPos.y + dy_long * (mid - a.screenPos.x);
    for (float x = mid; x < right; x++) {
        bottom = round(y1);
        top = round(y2);
        utils::sortAndClamp(bottom, top, zBuffer.height - 1);
        float cameraY = cam.getCameraYFromPixel(x, zBuffer.width);
        float depth;
        for (int y = bottom; y <= top; y++) {
            // calculate depth
            float cameraZ = cam.getCameraZFromPixel(y, zBuffer.height);
            float cameraX = 1;
            depth = (d1 / (normal.x * cameraX + normal.y * cameraY + normal.z * cameraZ)) * sqrt(cameraX * cameraX + cameraY * cameraY + cameraZ * cameraZ);
            if (depth < 0) {
                depth = 0;
            }
            if (depth < zBuffer.getDepth(x, y)) {
                float depthBias = std::max(0.05 * (1 - abs(normal.x)), 0.001);
                depthBias = 0;
                zBuffer.setDepth(x, y, depth + depthBias);
            }
        }
        y1 += dy2;
        y2 += dy_long;
    }
}
void Light::fillZBuffer(std::vector<Triangle> &triangles) {
    for (Triangle &triangle : triangles) {
        getTrianglePerspectiveFromLight(triangle);
    }
}
float Light::amountLit(Vec3 &vec, float& vecToLightMagInv) {
    Point p(vec);
    p.calculateCameraPos(cam);
    p.calculateProjectedPos();
    p.calculateScreenPos(cam, zBuffer.width, zBuffer.height);
    int x = round(p.screenPos.x);
    int y = round(p.screenPos.y);
    float lightingLevel = 0;
    int offset = 2;
    for (int i = x - offset; i <= x + offset; i++) {
        for (int j = y - offset; j <= y + offset; j++) {
            if (i < 0 || i >= zBuffer.width || j < 0 || j >= zBuffer.height) {
                continue;
            }
            if (p.distToCamera <= zBuffer.getDepth(i, j)) {
                lightingLevel += 1;
            }
        }
    }
    lightingLevel *= filteringAreaInv;
    lightingLevel *= luminosity;
    lightingLevel *= vecToLightMagInv * vecToLightMagInv;
    lightingLevel = std::min(lightingLevel, 1.0f);
    return lightingLevel;
}


//-----------------------------------------------------------------------------------
// IMPLEMENTATION OF "utils"
void utils::sortPair(float &toLower, float &toHigher) {
    if (toLower > toHigher) {
        std::swap(toLower, toHigher);
    }
}
void utils::clampToRange(float &value, float min, float max) {
    if (value < min) {
        value = min;
    } else if (value > max) {
        value = max;
    }
}
void utils::clampToRange(float &value, float max) {
    if (value < 0) {
        value = 0;
    } else if (value > max) {
        value = max;
    }
}
void utils::sortAndClamp(float &toLower, float &toHigher, float min, float max) {
    sortPair(toLower, toHigher);
    clampToRange(toLower, min, max);
    clampToRange(toHigher, min, max);
}
void utils::sortAndClamp(float &toLower, float &toHigher, float max) {
    if (toLower > toHigher) {
        std::swap(toLower, toHigher);
    }
    if (toLower < 0) {
        toLower = 0;
    }
    if (toHigher > max) {
        toHigher = max;
    }
}
void utils::sortPair(int &toLower, int &toHigher) {
    if (toLower > toHigher) {
        std::swap(toLower, toHigher);
    }
}
void utils::clampToRange(int &value, int max) {
    if (value < 0) {
        value = 0;
    } else if (value > max) {
        value = max;
    }
}
void utils::sortAndClamp(int &toLower, int &toHigher, int max) {
    if (toLower > toHigher) {
        std::swap(toLower, toHigher);
    }
    if (toLower < 0) {
        toLower = 0;
    }
    if (toHigher > max) {
        toHigher = max;
    }
}
int utils::getSceneMetaDataSize() {
    int size = 0;
    size += 1; // this value holds the number of objects
    size += graphics::Object3D::objects.size(); // each of these values holds the number of triangles in each object
    return size;
}
int* utils::getSceneMetaDataBuffer() {
    int size = getSceneMetaDataSize();
    int* buffer = new int[size];
    int index = 0;
    buffer[index++] = graphics::Object3D::objects.size();
    for (auto& object : graphics::Object3D::objects) {
        buffer[index++] = object.triangles.size();
    }
    return &buffer[0];
}
int* utils::getScenePosDataBuffer() {
    int size = 0;
    for (auto& object : graphics::Object3D::objects) {
        size += object.triangles.size() * 9;
    }
    float* buffer = new float[size];
    int index = 0;
    for (auto& object : graphics::Object3D::objects) {
        for (auto& triangle : object.triangles) {
            buffer[index++] = triangle.p1.absolutePos.x;
            buffer[index++] = triangle.p1.absolutePos.y;
            buffer[index++] = triangle.p1.absolutePos.z;
            buffer[index++] = triangle.p2.absolutePos.x;
            buffer[index++] = triangle.p2.absolutePos.y;
            buffer[index++] = triangle.p2.absolutePos.z;
            buffer[index++] = triangle.p3.absolutePos.x;
            buffer[index++] = triangle.p3.absolutePos.y;
            buffer[index++] = triangle.p3.absolutePos.z;
        }
    }
    return (int*) &buffer[0];
}
int* utils::getSceneColorDataBuffer() {
    int size = 0;
    for (auto& object : graphics::Object3D::objects) {
        size += object.triangles.size() * 3;
    }
    int* buffer = new int[size];
    int index = 0;
    for (auto& object : graphics::Object3D::objects) {
        for (auto& triangle : object.triangles) {
            buffer[index++] = triangle.r;
            buffer[index++] = triangle.g;
            buffer[index++] = triangle.b;
        }
    }
    return &buffer[0];
}
int* utils::setSceneDataBuffer(int size) {
    int* buffer = new int[size];
    return &buffer[0];
}
void utils::loadScene(int metadata[], float posData[], int colorData[]) {
    graphics::Object3D::objects.clear();
    
    int numObjects = metadata[0];
    int posIndex = 0;
    int colorIndex = 0;

    for (int i = 0; i < numObjects; i++) {
        int numTriangles = metadata[i + 1];
        graphics::Object3D object;
        for (int j = 0; j < numTriangles; j++) {
            graphics::Triangle triangle;
            triangle.p1.absolutePos.x = posData[posIndex++];
            triangle.p1.absolutePos.y = posData[posIndex++];
            triangle.p1.absolutePos.z = posData[posIndex++];
            triangle.p2.absolutePos.x = posData[posIndex++];
            triangle.p2.absolutePos.y = posData[posIndex++];
            triangle.p2.absolutePos.z = posData[posIndex++];
            triangle.p3.absolutePos.x = posData[posIndex++];
            triangle.p3.absolutePos.y = posData[posIndex++];
            triangle.p3.absolutePos.z = posData[posIndex++];
            triangle.r = colorData[colorIndex++];
            triangle.g = colorData[colorIndex++];
            triangle.b = colorData[colorIndex++];
            object.triangles.push_back(triangle);
        }
        graphics::Object3D::objects.push_back(object);
    }

    for (graphics::Light &l : graphics::Light::lights) {
        l.zBuffer.clear();
        for (graphics::Object3D &o : graphics::Object3D::objects) {
            l.fillZBuffer(o.triangles);
        }
    }

}