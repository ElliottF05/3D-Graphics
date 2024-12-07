#include "vec3.h"

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
Vec3 Vec3::operator*(const Vec3& other) const {
    return Vec3(this->x * other.x, this->y * other.y, this->z * other.z);
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
Vec3 Vec3::operator-() const {
    return Vec3(-this->x, -this->y, -this->z);
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
float Vec3::length() const {
    return std::sqrt(this->x * this->x + this->y * this->y + this->z * this->z);
}
float Vec3::lengthSquared() const {
    return this->x * this->x + this->y * this->y + this->z * this->z;
}
void Vec3::normalize() {
    float mag = this->length();
    if (mag == 0) {
        return;
    }
    (*this) /= mag;
}
Vec3 Vec3::normalized() const {
    float mag = this->length();
    if (mag == 0) {
        return Vec3();
    }
    return (*this) / mag;
}
float Vec3::angleWith(const Vec3 &other) const {
    return std::acos(this->dot(other) / (this->length() * other.length()));
}

// Operators where Vec3 is right-hand-side
Vec3 operator*(const float scalar, const Vec3& vec) {
    return Vec3(vec.x * scalar, vec.y * scalar, vec.z * scalar);
}
Vec3 operator/(const float scalar, const Vec3&vec) {
    float x = 1.0 / scalar;
    return Vec3(vec.x * x, vec.y * x, vec.z * x);
}
float dot(const Vec3& a, const Vec3& b) {
    return a.x * b.x + a.y * b.y + a.z * b.z;
}

// ROTATION
void Vec3::rotateZ(float thetaZ) {
    Vec3 orig = *this;

    this->x = orig.x * std::cos(thetaZ) - orig.y * std::sin(thetaZ);
    this->y = orig.x * std::sin(thetaZ) + orig.y * std::cos(thetaZ);
}
void Vec3::rotateY(float thetaY) {
    Vec3 orig = *this;

    this->x = orig.x * std::cos(thetaY) - orig.z * std::sin(thetaY);
    this->z = orig.x * std::sin(thetaY) + orig.z * std::cos(thetaY);
}
void Vec3::rotate(float thetaZ, float thetaY) {
    rotateZ(thetaZ);
    rotateY(thetaY);
}

// OTHER OPERATIONS
std::string Vec3::toString() const {
    return std::to_string(this->x) + ", " + std::to_string(this->y) + ", " + std::to_string(this->z);
}
bool Vec3::nearZero() const {
    const float s = 1e-8;
    return (std::fabs(this->x) < s) && (std::fabs(this->y) < s) && (std::fabs(this->z) < s);
}

// STATIC METHODS
Vec3 Vec3::random() {
    return Vec3(randomFloat(), randomFloat(), randomFloat());
}
Vec3 Vec3::random(float min, float max) {
    return Vec3(randomFloat(min, max), randomFloat(min, max), randomFloat(min, max));
}
Vec3 Vec3::randomUnitVector() {
    while (true) {
        Vec3 v = Vec3::random(-1, 1);
        float lengthSquared = v.lengthSquared();
        if (1e-80 <= lengthSquared && lengthSquared <= 1) {
            return v / std::sqrt(lengthSquared);
        }
    }
}
Vec3 Vec3::randomOnHemishpere(Vec3& normal) {
    Vec3 v = Vec3::randomUnitVector();
    if (normal.dot(v) > 0.0f) {
        return v;
    } else {
        return v * -1;
    }
}
Vec3 Vec3::reflect(const Vec3& vec, const Vec3& normal) {
    return vec - 2 * vec.dot(normal) * normal;
}
Vec3 Vec3::refract(const Vec3 &rayIn, const Vec3 &normal, float n1, float n2) {
    float n1Overn2 = n1 / n2;
    float cosTheta = std::fmin((-1 * rayIn).dot(normal), 1.0f);
    Vec3 rayOutPerp = n1Overn2 * (rayIn + cosTheta * normal);
    Vec3 rayOutParallel = -1 * normal * std::sqrt(std::fabs(1.0f - rayOutPerp.lengthSquared()));
    return rayOutPerp + rayOutParallel;
}