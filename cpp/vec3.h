#pragma once

#include <string>
#include <cmath>
#include "utils.h"

struct Vec3 {
    float x,y,z;

    // constructors
    Vec3(float x, float y, float z);
    Vec3();

    // operations
    Vec3 operator+(const Vec3& other) const;
    Vec3 operator-(const Vec3& other) const;
    Vec3 operator*(const float scalar) const;
    Vec3 operator/(const float scalar) const;
    Vec3 operator*(const Vec3& other) const;
    Vec3& operator+=(const Vec3& vec);
    Vec3& operator-=(const Vec3& vec);
    Vec3& operator*=(const float scalar);
    Vec3& operator/=(const float scalar);
    Vec3 operator-() const;
    Vec3 cross(const Vec3& other) const;
    float dot(const Vec3& other) const;
    float length() const;
    float lengthSquared() const;
    void normalize();
    Vec3 normalized() const;
    float angleWith(const Vec3& other) const;

    // rotation
    void rotateZ(float thetaZ);
    void rotateY(float thetaY);
    void rotate(float thetaZ, float thetaY);

    // other operations
    std::string toString() const;
    bool nearZero() const;

    // static methods
    static Vec3 random();
    static Vec3 random(float min, float max);
    static Vec3 randomUnitVector();
    static Vec3 randomOnHemishpere(Vec3& normal);
    static Vec3 reflect(const Vec3& vec, const Vec3& normal);
    static Vec3 refract(const Vec3& rayIn, const Vec3& normal, float n1, float n2);
};
// extra operators for Vec3
Vec3 operator*(const float scalar, const Vec3& vec);
Vec3 operator/(const float scalar, const Vec3& vec);
float dot(const Vec3& a, const Vec3& b);