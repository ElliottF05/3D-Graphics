#include "graphics.h"
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


