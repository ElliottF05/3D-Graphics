#include <SFML/Graphics/CircleShape.hpp>
#include <SFML/Graphics/RectangleShape.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/System/Vector2.hpp>
#include <SFML/Window/Window.hpp>
#include <SFML/Window/WindowStyle.hpp>
#include <cmath>
#include <iostream>
#include <chrono>
#include <thread>
#include "sfml/Graphics.hpp"


// Function prototype declaration
void drawLine(sf::RenderWindow& window, float x1, float y1, float x2, float y2);
void drawPoint(sf::RenderWindow& window, const sf::Vector2f& pos);


// Classes / Structs
struct Vec3;
struct Camera;
struct displayVec;

struct Vec3 {
    float x,y,z;

    Vec3(float x, float y, float z) {
        this->x = x;
        this->y = y;
        this->z = z;
    }

    Vec3() {
        this->x = 0;
        this->y = 0;
        this->z = 0;
    };

    Vec3(const Vec3& other) {
        this->x = other.x;
        this->y = other.y;
        this->z = other.z;
    }

    void add(const Vec3& other) {
        this->x += other.x;
        this->y += other.y;
        this->z += other.z;
    };

    void subtract(const Vec3& other) {
        this->x -= other.x;
        this->y -= other.y;
        this->z -= other.z;
    }

    void scalarMult(float k) {
        this->x *= k;
        this->y *= k;
        this->z *= k;
    }

    void rotateZ(float thetaZ) {
        Vec3 orig = Vec3(*this);

        this->x = orig.x * cos(thetaZ) - orig.y * sin(thetaZ);
        this->y = orig.x * sin(thetaZ) + orig.y * cos(thetaZ);
    }

    void rotateY(float thetaY) {
        Vec3 orig = Vec3(*this);

        this->x = orig.x * cos(thetaY) - orig.z * sin(thetaY);
        this->z = orig.x * sin(thetaY) + orig.z * cos(thetaY);
    }

    void rotate(float thetaZ, float thetaY) {
        // TODO: check this, might not work at all lmao 
        float angleFromX = atan(this->y / this->x);
        //std::cout << "angleFromX: " << angleFromX << "\n";
        rotateZ(-angleFromX);
        //std::cout << "after subtracting angle from x: " << toString(); 
        rotateY(thetaY);
        rotateZ(angleFromX + thetaZ);
        //std::cout << "after rotations: " << toString();
    }

    displayVec toPlaneCoords(const Camera& cam);
    sf::Vector2f toScreenCoords(const Camera& cam, sf::RenderWindow& window);
    void draw(const Camera& cam, sf::RenderWindow& window);

    std::string toString() {
        return std::to_string(this->x) + ", " + std::to_string(this->y) + ", " + std::to_string(this->z) + "\n";
    }

    static Vec3 add(const Vec3& v1, const Vec3& v2) {
        return Vec3(v1.x + v2.x, v1.y + v2.y, v1.z + v2.z);
    }
};

struct Camera {
    Vec3 pos;
    float thetaY, thetaZ, fov, fov_rad;

    Camera(Vec3 pos, float thetaY, float thetaZ, float fov) {
        this->pos = pos;
        this->thetaY = thetaY;
        this->thetaZ = thetaZ;
        this->fov = fov;
        this->fov_rad = fov * M_PI / 180.0;
    };
};

struct displayVec : Vec3 {
    bool inView;

    displayVec(float x, float y, bool inView) : Vec3(x, y, 0) {
        this->inView = inView;
    }
};

displayVec Vec3::toPlaneCoords(const Camera& cam) {
    Vec3 rotated = Vec3(*this);

    // std::cout << toString();

    rotated.subtract(cam.pos);
    rotated.rotate(-cam.thetaZ, -cam.thetaY);

    // std::cout << rotated.toString();

    return displayVec(rotated.y / rotated.x, rotated.z / rotated.x, (rotated.x > 0) ? true : false);
}

sf::Vector2f Vec3::toScreenCoords(const Camera& cam, sf::RenderWindow& window) {
    displayVec planeCoords = toPlaneCoords(cam);
    // std::cout << "planeCoords: " << planeCoords.x << ", " << planeCoords.y << "\n";
    float maxPlaneCoordValue = tan(0.5 * cam.fov_rad);

    float x, y;
    if (planeCoords.inView) {
        x = window.getSize().x * (planeCoords.x / maxPlaneCoordValue + 0.5);
        y = 0.5 * window.getSize().y - window.getSize().x * planeCoords.y / maxPlaneCoordValue;
    } else {
        x = 0;
        y = 0;
    }

    return sf::Vector2f(x, y);
}

void Vec3::draw(const Camera& cam, sf::RenderWindow& window) {
    drawPoint(window, toScreenCoords(cam, window));
}


int main(int, char**){

    sf::ContextSettings settings;
    settings.antialiasingLevel = 8;

    sf::RenderWindow window(sf::VideoMode(800, 600), "My window", sf::Style::Default, settings);
    window.setFramerateLimit(60); // call it once, after creating the window

    //testing some vectors
    Camera cam = Camera(Vec3(0, 0, 0), 0, 0, 90);
    Vec3 v1 = Vec3(1,0,0);
    Vec3 v2 = Vec3(10,2,0);
    Vec3 v3 = Vec3(10,0,3);

    // run the program as long as the window is open
    while (window.isOpen()) {
        cam.thetaZ += 0.01;

        // check all the window's events that were triggered since the last iteration of the loop
        sf::Event event;
        while (window.pollEvent(event)) {
            // "close requested" event: we close the window
            if (event.type == sf::Event::Closed)
                window.close();
        }

        // clear the window with black color
        window.clear(sf::Color::Black);

        // draw everything here...
        // window.draw(...);
        // drawLine(window, 200.f, 50.f, 500.f, 500.f);
        v1.draw(cam, window);
        v2.draw(cam, window);
        v3.draw(cam, window);

        // end the current frame
        window.display();

    std::this_thread::sleep_for(std::chrono::milliseconds(16));
    }

    return 0;
}


void drawLine(sf::RenderWindow& window, float x1, float y1, float x2, float y2) {
    sf::Vertex line[] =
        {
            sf::Vertex(sf::Vector2f(x1, y1)),
            sf::Vertex(sf::Vector2f(x2, y2))
        };

    window.draw(line, 2, sf::Lines);
}

void drawPoint(sf::RenderWindow& window, const sf::Vector2f& pos) {
    sf::CircleShape shape(5.f);
    shape.setPosition(pos);
    window.draw(shape);
    // std::cout << pos.x << ", " <<  pos.y << "\n";
}