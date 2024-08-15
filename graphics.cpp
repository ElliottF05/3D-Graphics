#include "graphics.h"
#include <cmath>
#include <cstdlib>
#include <iostream>
#include <vector>

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
    cameraPos.rotate(-cam.thetaZ, -cam.thetaY);
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
    screenPos.x = (0.5 * window.width) * (1 - projectedPos.x / cam.maxPlaneCoord) - 0.5;
    screenPos.y = (0.5 * window.height - projectedPos.y / cam.maxPlaneCoord * 0.5 * window.width) - 0.5;
}
void Point::calculateScreenPos(const Camera& cam, const int width, const int height) {
    screenPos.x = (0.5 * width) * (1 - projectedPos.x / cam.maxPlaneCoord) - 0.5;
    screenPos.y = (0.5 * height - projectedPos.y / cam.maxPlaneCoord * 0.5 * width) - 0.5;
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
std::vector<Triangle> Triangle::triangles; // defining the static variable, TODO: check if this is right
Triangle::Triangle(Point p1, Point p2, Point p3) {
    this->p1 = p1;
    this->p2 = p2;
    this->p3 = p3;
    this->absoluteNormal = (p2.absolutePos - p1.absolutePos).cross(p3.absolutePos - p1.absolutePos);
    absoluteNormal.normalize();
    this->r = std::rand() % 256;
    this->g = std::rand() % 256;
    this->b = std::rand() % 256;
}
Triangle::Triangle(Vec3 p1, Vec3 p2, Vec3 p3) : p1(p1), p2(p2), p3(p3) {
    absoluteNormal = (p2 - p1).cross(p3 - p1);
    absoluteNormal.normalize();
    this->r = std::rand() % 256;
    this->g = std::rand() % 256;
    this->b = std::rand() % 256;
}
Triangle::Triangle() {}

// METHODS
void Triangle::draw(const Camera& cam, Window& window) {
    Vec3 toCam = cam.pos - p1.absolutePos;
    if (absoluteNormal.dot(toCam) < 0) {
        absoluteNormal *= -1;
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

        window.drawTriangle(*this, cam);
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

        Triangle t(*front[1], *behind[0], behind2);
        t.absoluteNormal = absoluteNormal;
        t.cameraNormal = cameraNormal;
        t.r = r;
        t.g = g;
        t.b = b;

        window.drawTriangle(*this, cam);
        window.drawTriangle(t, cam);
    } else {
        front[0]->calculateScreenPos(cam, window);
        front[1]->calculateScreenPos(cam, window);
        front[2]->calculateScreenPos(cam, window);

        window.drawTriangle(*this, cam);
    }
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
}
Camera::Camera() {
    this->fov = 90;
    this->fov_rad = 90 * M_PI / 180;
    this->maxPlaneCoord = tan(fov_rad / 2);
    this->direction = Vec3(1,0,0);
    this->floorDirection = Vec3(1,0,0);
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
    direction.rotate(thetaZ, thetaY);
    floorDirection.rotate(thetaZ, 0);
}
float Camera::getCameraYFromPixel(int x, int width) const {
    return - maxPlaneCoord * (x - (0.5 * width) + 0.5) / (0.5 * width);
}
float Camera::getCameraZFromPixel(int y, int height) const {
    return - maxPlaneCoord * (y - (0.5 * height) + 0.5) / (0.5 * height);
}


//-----------------------------------------------------------------------------------
// IMPLEMENTATION OF "PixelArray"

// CONSTRUCTOR
PixelArray::PixelArray(int width, int height) {
    this->width = width;
    this->height = height;
    this->data = std::vector<int>(width * height * 3, 0);
}

// METHODS
int PixelArray::getIndex(int x, int y) {
    if (x < 0 || x >= width || y < 0 || y >= height) {
        std::cout << "PixelArray::getIndex() failed, pixel coordinates out of bounds. INPUTS: x = " << x << 
        ", y = " << y << std::endl; 
        throw "pixel coordinates out of bounds";
    }
    return ((this->width * y) + x) * 3;
}
void PixelArray::setPixel(int x, int y, int color) {
    if (color < 0 || color > 255) {
        std::cout << "PixelArray::setPixel() failed, color value out of bounds. INPUTS: color = " << color << std::endl;
        throw "color value out of bounds";
    }
    int index = this->getIndex(x, y);
    this->data[index] = color;
    this->data[index+1] = color;
    this->data[index+2] = color;
}
void PixelArray::setPixel(int x, int y, int r, int g, int b) {
    if (r < 0 || g < 0 || b < 0 || r > 255 || g > 255 || b > 255) {
        std::cout << "PixelArray::setPixel() failed, color value out of bounds. INPUTS: r, g, b = " << r << ", " << g << ", " << b << std::endl;
        throw "color value out of bounds";
    }
    int index = this->getIndex(x, y);
    this->data[index] = r;
    this->data[index+1] = g;
    this->data[index+2] = b;
}
int PixelArray::getPixelMonocolor(int x, int y) {
    int index = this->getIndex(x, y);
    return this->data[index];
}
std::vector<int> PixelArray::getPixel(int x, int y) {
    int index = this->getIndex(x, y);
    std::vector<int> v = {this->data[index], this->data[index+1], this->data[index+2]};
    return v;
}
void PixelArray::clear() {
    for (int i = 0; i < data.size(); i++) {
        data[i] = 0;
    }
}


//-----------------------------------------------------------------------------------
// IMPLEMENTATION OF "ZBuffer"

// CONSTRUCTOR
ZBuffer::ZBuffer(int width, int height) {
    this->width = width;
    this->height = height;
    data = std::vector<float>(width * height, 99999);
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
    data[index] = depth;
}
float ZBuffer::getDepth(int x, int y) {
    int index = getIndex(x, y);
    return data[index];
}
void ZBuffer::clear() {
    for (int i = 0; i < data.size(); i++) {
        data[i] = 99999;
    }
}


//-----------------------------------------------------------------------------------
// IMPLEMENTATION OF "Window"

// CONSTRUCTOR
Window::Window(int width, int height)
 : pixelArray(width, height), zBuffer(width, height) {
    this->width = width;
    this->height = height;
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
void Window::drawTriangle(Triangle &triangle, const Camera& cam) {
    Point a, b, c;
    a = triangle.p1;
    b = triangle.p2;
    c = triangle.p3;

    // equation for plane
    Vec3 normal = (a.cameraPos - b.cameraPos).cross(a.cameraPos - c.cameraPos);
    if (normal.x < 0) {
        normal *= -1;
    }
    normal.normalize();
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
    utils::clampToRange(left, width - 1);
    utils::clampToRange(mid, width - 1);
    utils::clampToRange(right, width - 1);

    // std::cout << left << ", " << mid << ", " << right << "\n";

    float y1, y2; // y1 for shorter line segment, y2 for longer line segment
    int bottom, top;
    y1 = a.screenPos.y + dy1 * (left - a.screenPos.x);
    y2 = a.screenPos.y + dy_long * (left - a.screenPos.x);
    for (float x = left; x < mid; x++) {
        bottom = round(y1);
        top = round(y2);
        utils::sortAndClamp(bottom, top, height - 1);
        float cameraY = cam.getCameraYFromPixel(x, width);
        float depth;
        for (int y = bottom; y <= top; y++) {
            // calculate depth
            float cameraZ = cam.getCameraZFromPixel(y, height);
            float cameraX = 1;
            depth = (d1 / (normal.x * cameraX + normal.y * cameraY + normal.z * cameraZ)) * sqrt(cameraX * cameraX + cameraY * cameraY + cameraZ * cameraZ);
            if (depth < 0) {
                if (depth < -0.1) {
                    // std::cout << "ERROR: depth < -0.1, depth = " << depth << std::endl;
                }
                depth = 0;
            }
            if (depth < zBuffer.getDepth(x, y)) {
                zBuffer.setDepth(x, y, depth);

                Vec3 vec(cameraX, cameraY, cameraZ);
                vec.normalize();
                vec *= depth;
                vec.rotateY(cam.thetaY);
                vec.rotateZ(cam.thetaZ);
                vec += cam.pos;

                float proportionInShadow = Light::lights[0].amountLit(vec);
                float angleLighting = Light::lights[0].cam.direction.dot(triangle.absoluteNormal * -1);
                if (angleLighting < 0) {
                    angleLighting = 0;
                }
                float multiplier = 0.2 + 0.8 * proportionInShadow * angleLighting;
                pixelArray.setPixel(x, y, multiplier * triangle.r, multiplier * triangle.g, multiplier * triangle.b);
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
        utils::sortAndClamp(bottom, top, height - 1);
        float cameraY = cam.getCameraYFromPixel(x, width);
        float depth;
        for (int y = bottom; y <= top; y++) {
            // calculate depth
            float cameraZ = cam.getCameraZFromPixel(y, height);
            float cameraX = 1;
            depth = (d1 / (normal.x * cameraX + normal.y * cameraY + normal.z * cameraZ)) * sqrt(cameraX * cameraX + cameraY * cameraY + cameraZ * cameraZ);
            if (depth < 0) {
                depth = 0;
            }
            if (depth < zBuffer.getDepth(x, y)) {
                zBuffer.setDepth(x, y, depth);

                Vec3 vec(cameraX, cameraY, cameraZ);
                vec.normalize();
                vec *= depth;
                vec.rotateY(cam.thetaY);
                vec.rotateZ(cam.thetaZ);
                vec += cam.pos;

                float proportionInShadow = Light::lights[0].amountLit(vec);
                float angleLighting = Light::lights[0].cam.direction.dot(triangle.absoluteNormal * -1);
                if (angleLighting < 0) {
                    angleLighting = 0;
                }
                float multiplier = 0.2 + 0.8 * proportionInShadow * angleLighting;
                pixelArray.setPixel(x, y, multiplier * triangle.r, multiplier * triangle.g, multiplier * triangle.b);
            }
        }
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
    int i = 0;
    int j = 0;
    while (i + 2 < pixelArray.data.size() && j + 3 < width * height * 4) {
        buffer[j] = pixelArray.data[i];
        buffer[j + 1] = pixelArray.data[i + 1];
        buffer[j + 2] = pixelArray.data[i + 2];
        buffer[j + 3] = 255;
        i += 3;
        j += 4;
    }
}


//-----------------------------------------------------------------------------------
// IMPLEMENTATION OF "Light"
std::vector<Light> Light::lights;

// CONSTRUCTORS
Light::Light(Point pos, float thetaZ, float thetaY) : zBuffer(4000, 4000) {
    Camera camera(pos.absolutePos, thetaZ, thetaY, atan(0.5) * 180 / M_PI);
    this->cam = camera;
}
Light::Light(Vec3 pos, float thetaZ, float thetaY) : zBuffer(4000, 4000) {
    Camera camera(pos, thetaZ, thetaY, atan(0.5) * 180 / M_PI);
    this->cam = camera;
}

// METHODS
void Light::getTrianglePerspectiveFromLight(Triangle triangle) {
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
                float depthBias = std::max(0.05 * (1 - abs(normal.x)), 0.003);
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
                float depthBias = std::max(0.05 * (1 - abs(normal.x)), 0.003);
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
float Light::amountLit(Vec3 &vec) {
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
    lightingLevel /= (2 * offset + 1) * (2 * offset + 1);
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