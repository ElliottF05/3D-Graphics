#pragma once

#include <SFML/Graphics/RenderWindow.hpp>
#include <vector>

namespace graphics {

// DECLARING STRUCTS;
struct Vec3;

struct Point;
struct Line;
struct Triangle;

struct Camera;
struct World;

struct PixelArray;
struct ZBuffer;
struct Window;


// DECLARING STRUCT METHODS AND INSTANCE VARIABLES
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
    Vec3& operator+=(const Vec3& vec);
    Vec3& operator-=(const Vec3& vec);
    Vec3& operator*=(const float scalar);
    Vec3& operator/=(const float scalar);
    Vec3 cross(const Vec3& other) const;
    float dot(const Vec3& other) const;
    float mag() const;
    float angleWith(const Vec3& other) const;

    // specific functions
    void rotateZ(float thetaZ);
    void rotateY(float thetaY);
    void rotate(float thetaZ, float thetaY);

    std::string toString();
};
// extra operators for Vec3
Vec3 operator*(const float scalar, const Vec3& vec);
Vec3 operator/(const float scalar, const Vec3& vec);

struct Point {
    Vec3 absolutePos, cameraPos, projectedPos, screenPos;
    float distToCamera;

    Point();
    Point(Vec3 absolutePos);

    void calculateCameraPos(const Camera& cam);
    void calculateProjectedPos();
    void calculateScreenPos(const Window& window);

    void drawOnScreen(const Window& window);

    void draw(const Camera& cam, const Window& window);
};

struct Line {
    Point p1, p2;

    Line();
    Line(Vec3 p1, Vec3 p2);

    void draw(const Camera& cam, const Window& window);
};

struct Triangle {
    Point p1, p2, p3;

    Triangle();
    Triangle(Vec3 p1, Vec3 p2, Vec3 p3);

    void draw(const Camera& cam, const Window& window);
};


struct Camera {
    Vec3 pos;
    float thetaZ, thetaY, fov;
    Vec3 direction;
    Vec3 floorDirection;

    Camera();
    Camera(Vec3 pos, float thetaZ, float thetaY, float fov);

    void moveRelative(float forward, float sideward);
    void rotate(float thetaZ, float thetaY);
};

struct World { // TODO: incomplete and unused
    Camera cam;
    Vec3 sunDirection;
};


struct PixelArray {
    int width, height;
    std::vector<std::vector<int> > data;
};

struct ZBuffer {
    int width, height;
    std::vector<std::vector<int> > data;
};

struct Window {
    int width, height;
    PixelArray pixelArray;
    ZBuffer zBuffer;
    sf::RenderWindow& sfmlWindow; // implementation specific
};




}