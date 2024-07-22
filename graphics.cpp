#include "graphics.h"
#include <SFML/Config.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Graphics/Sprite.hpp>
#include <SFML/Graphics/Texture.hpp>
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
    screenPos.x = (0.5 * window.width) * (1 - projectedPos.x / cam.maxPlaneCoord);
    screenPos.y = 0.5 * window.height - projectedPos.y / cam.maxPlaneCoord * 0.5 * window.width;
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
}
Triangle::Triangle(Vec3 p1, Vec3 p2, Vec3 p3) : p1(p1), p2(p2), p3(p3) {}
Triangle::Triangle() {}

// METHODS
// TODO: add draw() method


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
        std::cout << "PixelArray::getIndex() failed, pixel coordinates out of bound. INPUTS: x = " << x << 
        ", y = " << y << "\n"; 
        throw "pixel coordinates out of bounds";
    }
    return ((this->width * y) + x) * 3;
}
void PixelArray::setPixel(int x, int y, int color) {
    if (color < 0 || color > 255) {
        throw "color value out of bounds";
    }
    int index = this->getIndex(x, y);
    this->data[index] = color;
    this->data[index+1] = color;
    this->data[index+2] = color;
}
void PixelArray::setPixel(int x, int y, int r, int g, int b) {
    if (r < 0 || g < 0 || b < 0 || r > 255 || g > 255 || b > 255) {
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
        throw "pixel coordinates out of bounds";
    }
    return ((this->width * y) + x) * 3;
}
void ZBuffer::setDepth(int x, int y, float depth) {
    if (depth < 0) {
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
Window::Window(int width, int height, sf::RenderWindow& sfmlWindow)
 : pixelArray(width, height), zBuffer(width, height), sfmlWindow(sfmlWindow) {
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

    float y = a.screenPos.y + dy * (startVal - a.screenPos.x);
    for (int x = startVal; x <= endVal; x++) {
        int yVal = round(y);
        if (yVal >= 0 && yVal < height) {
            pixelArray.setPixel(x, yVal, 255);
        }
        y += dy;
    }
    
}
void Window::drawTriangle(Triangle &triangle) {
    Point a, b, c;
    a = triangle.p1;
    b = triangle.p2;
    c = triangle.p3;

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

    int left = round(a.screenPos.x);
    int mid = round(b.screenPos.x);
    int right = round(c.screenPos.x);
    left = std::max(0, left);
    right = std::min(width - 1, right);
    mid = std::max(0, mid);
    mid = std::min(width - 1, mid);

    std::cout << left << ", " << mid << ", " << right << "\n";

    float y1, y2; // y1 for shorter line segment, y2 for longer line segment
    int bottom, top;
    y1 = a.screenPos.y + dy1 * (left - a.screenPos.x);
    y2 = a.screenPos.y + dy_long * (left - a.screenPos.x);
    for (int x = left; x <= mid; x++) {
        bottom = round(y1);
        top = round(y2);
        if (top < bottom) {
            std::swap(top, bottom);
        }
        bottom = std::max(0, bottom);
        top = std::min(height - 1, top);
        for (int y = bottom; y <= top; y++) {
            pixelArray.setPixel(x, y, 255);
        }
        y1 += dy1;
        y2 += dy_long;
    }

    y1 = b.screenPos.y + dy2 * (mid - b.screenPos.x);
    y2 = a.screenPos.y + dy_long * (mid - a.screenPos.x);
    for (int x = mid; x <= right; x++) {
        bottom = round(y1);
        top = round(y2);
        if (top < bottom) {
            std::swap(top, bottom);
        }
        bottom = std::max(0, bottom);
        top = std::min(height - 1, top);
        for (int y = bottom; y <= top; y++) {
            pixelArray.setPixel(x, y, 255);
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

    // first convert to sfml Uint8 array

    auto t1 = std::chrono::high_resolution_clock::now();
    sf::Uint8 sfpixel[height * width * 4];
    // i gives index in pixelArray, j gives index in sfpixel[]
    for (int i = 0, j = 0; i < height * width * 3; j++) {
        if (j >= height * width * 4) {
            throw "index in sfpixel[] is out of bounds";
        }
        if ((j + 1) % 4 == 0) {
            sfpixel[j] = 255;
        } else {
            sfpixel[j] = static_cast<sf::Uint8>(pixelArray.data[i]);
            i++;
        }
    }

    // second, load to sf::Texture

    auto t2 = std::chrono::high_resolution_clock::now();

    sf::Texture texture;
    texture.create(width, height);
    texture.update(sfpixel);

    // load to sf::Sprinte
    sf::Sprite sprite;
    sprite.setTexture(texture);

    // draw and display
    sfmlWindow.draw(sprite);
    sfmlWindow.display();
    auto t3 = std::chrono::high_resolution_clock::now();
    auto pixelTime = std::chrono::duration_cast<std::chrono::microseconds>(t2 - t1);
    auto spriteTime = std::chrono::duration_cast<std::chrono::microseconds>(t3 - t2);
    // std::cout << "inside graphics - pixel time: " << pixelTime.count() << ", sprite time: " << spriteTime.count() << "\n";
}