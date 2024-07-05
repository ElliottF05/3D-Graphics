#pragma once

#include "2d.h"
#include <SFML/Graphics/RenderWindow.hpp>
#include <sfml/Graphics.hpp>

namespace wd {
    struct PixelArray;
}

namespace _3d {

    struct Camera;
    struct Vec3;
    struct Line;
    struct Triangle;

    struct Vec3 {
        float x,y,z,depth;

        Vec3(float x, float y, float z);
        Vec3();
        Vec3(const Vec3& other);

        void add(const Vec3& other);
        void subtract(const Vec3& other);
        void scalarMult(float k);

        Vec3 operator+(const Vec3& other) const;
        Vec3 operator-(const Vec3& other) const;
        Vec3 operator*(const float scalar) const;
        Vec3 operator/(const float scalar) const;
        bool operator==(const Vec3& other) const;

        Vec3 cross(const Vec3& other) const;
        float dot(const Vec3& other) const;
        float mag() const;
        float angleWith(const Vec3& other) const;

        void rotateZ(float thetaZ);
        void rotateY(float thetaY);
        void subtractAndRotate(const Camera&cam);
        void rotate(float thetaZ, float thetaY);
        void toPlaneCoords();
        void fullyToPlaneCoords(const Camera& cam);
        void toScreenCoords(const Camera& cam, sf::RenderWindow& window);

        void draw(const Camera& cam, sf::RenderWindow& window);

        std::string toString();
    };

    Vec3 operator*(const float scalar, const Vec3& vec);
    Vec3 operator/(const float scalar, const Vec3&vec);

    struct Camera {
        Vec3 pos;
        float thetaY, thetaZ, fov, fov_rad;

        Camera();
        Camera(Vec3 pos, float thetaY, float thetaZ, float fov);

        void setThetaY(float angle);
        void setThetaZ(float angle);

        Vec3 getUnitFloorVector();
    };

    struct Line {
        Vec3 p1;
        Vec3 p2;

        Line(Vec3& p1, Vec3& p2);

        void draw(const Camera& cam, sf::RenderWindow& window);
    };

    struct Triangle {
        static std::vector<Triangle*> triangles;
        Vec3 p1;
        Vec3 p2;
        Vec3 p3;
        Vec3 center;
        Vec3 norm;
        float distanceToCam;

        Triangle(Vec3 p1, Vec3 p2, Vec3 p3);

        void draw(const Camera& cam, sf::RenderWindow& window, wd::PixelArray& pixelArray);
        
        static bool compareByDistance(Triangle* t1, Triangle* t2);
        static void drawAll(const Camera& cam, sf::RenderWindow& window, wd::PixelArray& pixelArray);
    };

    struct World {
        Camera cam;
        Vec3 sunDirection;

        World(Camera cam, Vec3 sunDirection);
    };

}