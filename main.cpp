#include <SFML/Graphics/Image.hpp>
#include <SFML/Graphics/RectangleShape.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Graphics/Sprite.hpp>
#include <SFML/System/Vector2.hpp>
#include <SFML/Window/Cursor.hpp>
#include <SFML/Window/Event.hpp>
#include <SFML/Window/Keyboard.hpp>
#include <SFML/Window/Mouse.hpp>
#include <SFML/Window/VideoMode.hpp>
#include <SFML/Window/Window.hpp>
#include <SFML/Window/WindowStyle.hpp>
#include <__chrono/duration.h>
#include <cmath>
#include <iostream>
#include <chrono>
#include <thread>
#include <sfml/Graphics.hpp>
#include <vector>
#include "2d.h"
#include "3d.h"
#include "window.h"
#include "graphics.h"


int main(int, char**){

    // creating window
    sf::RenderWindow sfmlwindow(sf::VideoMode(800, 800), "My window", sf::Style::Default);
    sfmlwindow.setFramerateLimit(60);
    graphics::Window window(800, 800, sfmlwindow);

    // setting mouse to screen center
    sf::Vector2i screenCenter = sf::Vector2i(sf::VideoMode::getDesktopMode().width / 2, sf::VideoMode::getDesktopMode().height / 2);
    sf::Mouse::setPosition(screenCenter);


    // Setting up simulation necessities
    bool running = true;
    graphics::Camera cam;


    // TESTING
    graphics::Point p1(10, 1, 1), p2(10, -1, -1), p3;
    graphics::Line l1(p1, p2);


    // run the program as long as the window is open
    while (running) {
        auto time1 = std::chrono::high_resolution_clock::now();

        // check all the window's events that were triggered since the last iteration of the loop
        sf::Event event;
        while (window.sfmlWindow.pollEvent(event)) {
            // "close requested" event: we close the window
            if (event.type == sf::Event::Closed)
                running = false;
        }
        // if (sf::Keyboard::isKeyPressed(sf::Keyboard::Q)) {
        //     window.close();
        //     running = false;
        //     break;
        // }
        // if (sf::Keyboard::isKeyPressed(sf::Keyboard::Left)) {
        //     cam.thetaZ += 0.02;
        // }
        // if (sf::Keyboard::isKeyPressed(sf::Keyboard::Right)) {
        //     cam.thetaZ -= 0.02;
        // }
        // if (sf::Keyboard::isKeyPressed(sf::Keyboard::Up)) {
        //     cam.setThetaY(cam.thetaY + 0.02);
        // }
        // if (sf::Keyboard::isKeyPressed(sf::Keyboard::Down)) {
        //     cam.setThetaY(cam.thetaY - 0.02);
        // }
        // if (sf::Keyboard::isKeyPressed(sf::Keyboard::W)) {
        //     _3d::Vec3 v = cam.getUnitFloorVector();
        //     v.scalarMult(0.2);
        //     cam.pos.add(v);
        // }
        // if (sf::Keyboard::isKeyPressed(sf::Keyboard::S)) {
        //     _3d::Vec3 v = cam.getUnitFloorVector();
        //     v.scalarMult(-0.2);
        //     cam.pos.add(v);
        // }
        // if (sf::Keyboard::isKeyPressed(sf::Keyboard::D)) {
        //     _3d::Vec3 u = cam.getUnitFloorVector();
        //     _3d::Vec3 v = _3d::Vec3(u.y, -u.x, 0);
        //     v.scalarMult(0.2);
        //     cam.pos.add(v);
        // }
        // if (sf::Keyboard::isKeyPressed(sf::Keyboard::A)) {
        //     _3d::Vec3 u = cam.getUnitFloorVector();
        //     _3d::Vec3 v = _3d::Vec3(-u.y, u.x, 0);
        //     v.scalarMult(0.2);
        //     cam.pos.add(v);
        // }
        // if (sf::Keyboard::isKeyPressed(sf::Keyboard::Space)) {
        //     cam.pos.z += 0.1;
        // }
        // if (sf::Keyboard::isKeyPressed(sf::Keyboard::LShift)) {
        //     cam.pos.z -= 0.1;
        // }

        // sf::Vector2i mousePos = sf::Mouse::getPosition();
        // mousePos.y /= 3;
        // cam.setThetaZ(cam.thetaZ - (mousePos.x - screenCenter.x) / 400.0);
        // cam.setThetaY(cam.thetaY - (mousePos.y - screenCenter.y) / 400.0);
        // sf::Mouse::setPosition(screenCenter);




        auto time2 = std::chrono::high_resolution_clock::now();

        // TODO: basic testing for now
        l1.draw(cam, window);
        window.draw();

        auto time3 = std::chrono::high_resolution_clock::now();


        auto time4 = std::chrono::high_resolution_clock::now();
        auto frameTime = std::chrono::duration_cast<std::chrono::microseconds>(time4 - time1);
        auto pixelTime = std::chrono::duration_cast<std::chrono::microseconds>(time3 - time2);
        // std::cout << "frame time: " << frameTime.count() << ", pixel time: " << pixelTime.count() << "\n";

        // current data gives frame time around 27ms, with 21ms being drawing pixels to texture!!

    //std::this_thread::sleep_for(std::chrono::milliseconds(16));
    }

    return 0;
}