#pragma once

#include "2d.h"
#include <sfml/Graphics.hpp>

namespace _3d {

    struct Camera;

    struct Vec3 {
        float x,y,z;

        Vec3(float x, float y, float z);
        Vec3();
        Vec3(const Vec3& other);
        void add(const Vec3& other);
        void subtract(const Vec3& other);
        void scalarMult(float k);

        void rotateZ(float thetaZ);

        void rotateY(float thetaY);

        void rotate(float thetaZ, float thetaY);

        void toPlaneCoords(const Camera& cam);
        _2d::Vec2 toScreenCoords(const Camera& cam, sf::RenderWindow& window);
        void draw(const Camera& cam, sf::RenderWindow& window);

        std::string toString();
    };

    struct Camera {
        Vec3 pos;
        float thetaY, thetaZ, fov, fov_rad;

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
        Vec3 p1;
        Vec3 p2;
        Vec3 p3;

        Triangle(Vec3& p1, Vec3& p2, Vec3& p3);

        void draw(const Camera& cam, sf::RenderWindow& window);
    };

}