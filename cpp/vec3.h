#pragma once

#include <string>
#include <cmath>
#include "utils.h"

struct Vec3 {
    float x,y,z;

    // constructors
    Vec3(float x, float y, float z);
    Vec3();

    // operator overloading
    Vec3 operator+(const Vec3& other) const;
    Vec3 operator-(const Vec3& other) const;
    Vec3 operator*(const float scalar) const;
    Vec3 operator/(const float scalar) const;
    Vec3& operator+=(const Vec3& vec);
    Vec3& operator-=(const Vec3& vec);
    Vec3& operator*=(const float scalar);
    Vec3& operator/=(const float scalar);
    Vec3 cross(const Vec3& other) const;
    float dot(const Vec3& other) const;
    float length() const;
    float lengthSquared() const;
    void normalize();
    Vec3 normalized() const;
    float angleWith(const Vec3& other) const;

    // specific functions
    void rotateZ(float thetaZ);
    void rotateY(float thetaY);
    void rotate(float thetaZ, float thetaY);

    std::string toString() const;

    // static methods
    static Vec3 random();
    static Vec3 random(float min, float max);
    static Vec3 randomUnitVector();
    static Vec3 randomOnHemishpere(Vec3& normal);
};
// extra operators for Vec3
Vec3 operator*(const float scalar, const Vec3& vec);
Vec3 operator/(const float scalar, const Vec3& vec);
float dot(const Vec3& a, const Vec3& b);