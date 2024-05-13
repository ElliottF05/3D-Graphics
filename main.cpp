#include <SFML/Graphics/CircleShape.hpp>
#include <SFML/Graphics/RectangleShape.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/System/Vector2.hpp>
#include <SFML/Window/Window.hpp>
#include <SFML/Window/WindowStyle.hpp>
#include <iostream>
#include <format>
#include "sfml/Graphics.hpp"


// Function prototype declaration
void drawLine(sf::RenderWindow& window, float x1, float y1, float x2, float y2);
void drawPoint(sf::RenderWindow& window, const sf::Vector2f& pos);


// Classes / Structs
struct Vec3;
struct Camera;

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

    sf::Vector2f toScreenCoords(const Camera& cam);

    std::string toString() {
        return std::to_string(this->x) + ", " + std::to_string(this->y) + ", " + std::to_string(this->z) + "\n";
    }

    static Vec3 add(const Vec3& v1, const Vec3& v2) {
        return Vec3(v1.x + v2.x, v1.y + v2.y, v1.z + v2.z);
    }
};

struct Camera {
    Vec3 pos;
    float thetaY, thetaZ;

    Camera(Vec3 pos, float thetaY, float thetaZ) {
        this->pos = pos;
        this->thetaY = thetaY;
        this->thetaZ = thetaZ;
    };
};

sf::Vector2f Vec3::toScreenCoords(const Camera& cam) {
    Vec3 rotated = Vec3(*this);

    std::cout << toString();

    rotated.subtract(cam.pos);
    rotated.rotate(-cam.thetaZ, -cam.thetaY);

    std::cout << toString();

    // TODO: ADD FOV CALCULATIONS, remove placeholder mult
    int mult = 100;
    return sf::Vector2f(mult * rotated.y / rotated.x, mult * rotated.z / rotated.x);
}


int main(int, char**){
    std::cout << "Hello, from 3D-Graphics-Engine!\n";

    sf::ContextSettings settings;
    settings.antialiasingLevel = 8;

    sf::RenderWindow window(sf::VideoMode(800, 600), "My window", sf::Style::Default, settings);
    window.setFramerateLimit(60); // call it once, after creating the window

    //testing some vectors
    Camera cam = Camera(Vec3(0, 0, 0), 0, 0);
    Vec3 v1 = Vec3(1,0,0);
    Vec3 v2 = Vec3(10,2,0);
    Vec3 v3 = Vec3(5,0,3);

    // run the program as long as the window is open
    bool shouldRender = true;
    while (window.isOpen())
    {
        // check all the window's events that were triggered since the last iteration of the loop
        sf::Event event;
        while (window.pollEvent(event))
        {
            // "close requested" event: we close the window
            if (event.type == sf::Event::Closed)
                window.close();
        }

        // clear the window with black color
        if (shouldRender) {
            window.clear(sf::Color::Black);

            // draw everything here...
            // window.draw(...);
            // drawLine(window, 200.f, 50.f, 500.f, 500.f);

            drawPoint(window, v1.toScreenCoords(cam));
            drawPoint(window, v2.toScreenCoords(cam));
            drawPoint(window, v3.toScreenCoords(cam));

            // end the current frame
            window.display();
        }

        shouldRender = false;

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

void drawPoint(sf::RenderWindow& window, const sf::Vector2f& screenPos) {
    sf::Vector2f displayCoords = sf::Vector2f(window.getSize().x / 2.0 + screenPos.x, window.getSize().y / 2.0 - screenPos.y);
    sf::CircleShape shape(5.f);
    shape.setPosition(displayCoords);
    window.draw(shape);
}