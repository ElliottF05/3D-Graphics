#include <SFML/Graphics/CircleShape.hpp>
#include <SFML/Graphics/RectangleShape.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/System/Vector2.hpp>
#include <SFML/Window/Event.hpp>
#include <SFML/Window/Keyboard.hpp>
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
struct DisplayVec;

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

    DisplayVec toPlaneCoords(const Camera& cam);
    DisplayVec toScreenCoords(const Camera& cam, sf::RenderWindow& window);
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

struct DisplayVec : Vec3 {
    bool inFront;

    DisplayVec(float x, float y, bool inFront) : Vec3(x, y, 0) {
        this->inFront = inFront;
    }

    void draw(sf::RenderWindow& window) {
        drawPoint(window, sf::Vector2f(this->x, this->y));
    }
};

struct Line {
    Vec3 p1;
    Vec3 p2;

    Line(Vec3& p1, Vec3& p2) {
        this->p1 = p1;
        this->p2 = p2;
    }

    void draw(const Camera& cam, sf::RenderWindow& window) {
        DisplayVec v1 = p1.toScreenCoords(cam, window);
        DisplayVec v2 = p2.toScreenCoords(cam, window);

        if (!v1.inFront && !v2.inFront) {
            return;
        } else if (!v1.inFront) {
            v1.scalarMult(-1);
            v1.scalarMult(10);
        } else if (!v2.inFront) {
            v2.scalarMult(-1);
            v2.scalarMult(10);
        }

        drawLine(window, v1.x, v1.y, v2.x, v2.y);
    };
};

DisplayVec Vec3::toPlaneCoords(const Camera& cam) {
    Vec3 rotated = Vec3(*this);

    // std::cout << toString();

    rotated.subtract(cam.pos);
    rotated.rotate(-cam.thetaZ, -cam.thetaY);

    // std::cout << rotated.toString();

    return DisplayVec(rotated.y / rotated.x, rotated.z / rotated.x, (rotated.x > 0) ? true : false);
}
DisplayVec Vec3::toScreenCoords(const Camera& cam, sf::RenderWindow& window) {
    DisplayVec planeCoords = toPlaneCoords(cam);
    // std::cout << "planeCoords: " << planeCoords.x << ", " << planeCoords.y << "\n";
    float maxPlaneCoordValue = tan(0.5 * cam.fov_rad);

    float screenX = window.getSize().x * (planeCoords.x / maxPlaneCoordValue + 0.5);
    float screenY = 0.5 * window.getSize().y - window.getSize().x * planeCoords.y / maxPlaneCoordValue;

    return DisplayVec(screenX, screenY, planeCoords.inFront);
}
void Vec3::draw(const Camera& cam, sf::RenderWindow& window) {
    DisplayVec v = toScreenCoords(cam, window);
    if (v.inFront) {
        v.draw(window);
    }
}


int main(int, char**){

    sf::ContextSettings settings;
    settings.antialiasingLevel = 8;

    sf::RenderWindow window(sf::VideoMode(800, 600), "My window", sf::Style::Default, settings);
    window.setFramerateLimit(60); // call it once, after creating the window

    //testing some vectors
    Camera cam = Camera(Vec3(0, 0, 0), 0, 0, 90);
    Vec3 v1 = Vec3(10,0, -2);
    Vec3 v2 = Vec3(10,2,-2);
    Vec3 v3 = Vec3(11, 1, -2);
    Vec3 v4 = Vec3(10.5, 1, 2);
    Line l1 = Line(v1, v2);
    Line l2 = Line(v1, v3);
    Line l3 = Line(v1, v4);
    Line l4 = Line(v2, v3);
    Line l5 = Line(v2, v4);
    Line l6 = Line(v3, v4);

    // run the program as long as the window is open
    while (window.isOpen()) {

        // check all the window's events that were triggered since the last iteration of the loop
        sf::Event event;
        while (window.pollEvent(event)) {
            // "close requested" event: we close the window
            if (event.type == sf::Event::Closed)
                window.close();

        }

        if (sf::Keyboard::isKeyPressed(sf::Keyboard::Left)) {
            cam.thetaZ -= 0.02;
        }
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::Right)) {
            cam.thetaZ += 0.02;
        }

        // clear the window with black color
        window.clear(sf::Color::Black);

        // draw everything here...
        // window.draw(...);
        // drawLine(window, 200.f, 50.f, 500.f, 500.f);
        l1.draw(cam, window);
        l2.draw(cam, window);
        l3.draw(cam, window);
        l4.draw(cam, window);
        l5.draw(cam, window);
        l6.draw(cam, window);

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

void draw2DTriangle(sf::RenderWindow& window) {
    
}