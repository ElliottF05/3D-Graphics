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
#include <sfml/Graphics.hpp>
#include "2d.h"
#include "3d.h"


int main(int, char**){

    sf::ContextSettings settings;
    settings.antialiasingLevel = 8;

    sf::RenderWindow window(sf::VideoMode(800, 600), "My window", sf::Style::Default, settings);
    window.setFramerateLimit(60); // call it once, after creating the window

    //testing some vectors
    _3d::Camera cam = _3d::Camera(_3d::Vec3(0, 0, 0), 0, 0, 90);
    _3d::Vec3 v1 = _3d::Vec3(10,0, -2);
    _3d::Vec3 v2 = _3d::Vec3(10,2,-2);
    _3d::Vec3 v3 = _3d::Vec3(11, 1, -2);
    _3d::Vec3 v4 = _3d::Vec3(10.5, 1, 2);
    _3d::Line l1 = _3d::Line(v1, v2);
    _3d::Line l2 = _3d::Line(v1, v3);
    _3d::Line l3 = _3d::Line(v1, v4);
    _3d::Line l4 = _3d::Line(v2, v3);
    _3d::Line l5 = _3d::Line(v2, v4);
    _3d::Line l6 = _3d::Line(v3, v4);

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
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::Up)) {
            cam.thetaY += 0.02;
        }
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::Down)) {
            cam.thetaY -= 0.02;
        }
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::W)) {
            _3d::Vec3 v = cam.getUnitFloorVector();
            v.scalarMult(0.2);
            cam.pos.add(v);
        }
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::S)) {
            _3d::Vec3 v = cam.getUnitFloorVector();
            v.scalarMult(-0.2);
            cam.pos.add(v);
        }
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::A)) {
            _3d::Vec3 u = cam.getUnitFloorVector();
            _3d::Vec3 v = _3d::Vec3(u.y, -u.x, 0);
            v.scalarMult(0.2);
            cam.pos.add(v);
        }
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::D)) {
            _3d::Vec3 u = cam.getUnitFloorVector();
            _3d::Vec3 v = _3d::Vec3(-u.y, u.x, 0);
            v.scalarMult(0.2);
            cam.pos.add(v);
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