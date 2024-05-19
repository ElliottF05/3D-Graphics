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

        _2d::Vec2 toPlaneCoords(const Camera& cam);
        _2d::Vec2 toScreenCoords(const Camera& cam, sf::RenderWindow& window);
        void draw(const Camera& cam, sf::RenderWindow& window);

        std::string toString();
    };

    struct Camera {
        Vec3 pos;
        float thetaY, thetaZ, fov, fov_rad;

        Camera(Vec3 pos, float thetaY, float thetaZ, float fov);
    };

    struct Line {
        Vec3 p1;
        Vec3 p2;

        Line(Vec3& p1, Vec3& p2);

        void draw(const Camera& cam, sf::RenderWindow& window);
    };

}