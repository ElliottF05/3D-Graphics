#pragma once

#include <string>

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
    float mag() const;
    void normalize();
    float angleWith(const Vec3& other) const;

    // specific functions
    void rotateZ(float thetaZ);
    void rotateY(float thetaY);
    void rotate(float thetaZ, float thetaY);

    std::string toString() const;
};
// extra operators for Vec3
Vec3 operator*(const float scalar, const Vec3& vec);
Vec3 operator/(const float scalar, const Vec3& vec);