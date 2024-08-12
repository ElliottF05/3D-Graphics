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
#include <cstdlib>
#include <iostream>
#include <chrono>
#include <thread>
#include <sfml/Graphics.hpp>
#include <vector>
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
    graphics::Point p1(-5, 10, -1), p2(-5, -10, -1), p3(15, 0, -1), p4(1, -5, -1);
    graphics::Triangle t1(p1, p2, p3);
    graphics::Triangle::triangles.push_back(t1);

    graphics::Vec3 not_used(-10, 0, 10);
    graphics::Light l1(not_used, 0, -M_PI / 4.0);
    graphics::Light::lights.push_back(l1);

    for (graphics::Light &l : graphics::Light::lights) {
        l.fillZBuffer(graphics::Triangle::triangles);
    }

    std::vector<graphics::Line> floorGrid;
    for (int i = -5; i <= 5; i++) {
        graphics::Point p1(i, 5, -2), p2(i, -5, -2);
        graphics::Line a = *(new graphics::Line(p1, p2));
        graphics::Point p3(5, i, -2), p4(-5, i, -2);
        graphics::Line b = *(new graphics::Line(p3, p4));
        floorGrid.push_back(a);
        floorGrid.push_back(b);
    }


    // run the program as long as the window is open
    while (running) {
        auto time1 = std::chrono::high_resolution_clock::now();

        // check all the window's events that were triggered since the last iteration of the loop
        sf::Event event;
        while (window.sfmlWindow.pollEvent(event)) {
            // "close requested" event: we close the window
            if (event.type == sf::Event::Closed)
                running = false;
            if (event.type == sf::Event::KeyPressed) {
                if (event.key.code == sf::Keyboard::C) {
                    graphics::Vec3 a = cam.pos;
                    graphics::Vec3 b = cam.pos + graphics::Vec3(0.5 + (2.0 * std::rand()) / RAND_MAX, 0, 0);
                    graphics::Vec3 c = cam.pos + graphics::Vec3(0, 0.5 + (2.0 * std::rand()) / RAND_MAX, 0);
                    graphics::Vec3 d = cam.pos + graphics::Vec3(0, 0, 0.5 + (2.0 * std::rand()) / RAND_MAX);
                    graphics::Triangle t1(a, b, c);
                    graphics::Triangle t2(a, b, d);
                    graphics::Triangle t3(a,c,d);
                    graphics::Triangle t4(b, c, d);
                    graphics::Triangle::triangles.push_back(t1);
                    graphics::Triangle::triangles.push_back(t2);
                    graphics::Triangle::triangles.push_back(t3);
                    graphics::Triangle::triangles.push_back(t4);
                    
                    for (graphics::Light &l : graphics::Light::lights) {
                        l.fillZBuffer(graphics::Triangle::triangles);
                    }

                }
                if (event.key.code == sf::Keyboard::V) {
                    int iterations = 100;
                    std::vector<graphics::Vec3> prev(iterations);
                    std::vector<graphics::Vec3> curr(iterations);
                    bool onFirst = true;
                    for (float thetaY = -M_PI / 2.0; thetaY <= M_PI / 2.0; thetaY += M_PI / iterations) {
                        prev = curr;
                        curr.clear();
                        for (float thetaZ = 0; thetaZ <= 2 * M_PI; thetaZ += 2 * M_PI / iterations) {
                            graphics::Vec3 v(std::cos(thetaY) * std::cos(thetaZ), std::cos(thetaY) * std::sin(thetaZ), std::sin(thetaY));
                            v += cam.pos;
                            curr.push_back(v);
                        }
                        if (onFirst) {
                            onFirst = false;
                            continue;
                        }
                        for (int i = 0; i < prev.size(); i++) {
                            graphics::Triangle t1(curr[i], prev[i], curr[(i + 1) % iterations]);
                            graphics::Triangle t2(prev[i], prev[(i + 1) % iterations], curr[(i + 1) % iterations]);
                            t1.r = 255;
                            t1.g = 255;
                            t1.b = 255;
                            t2.r = 255;
                            t2.g = 255;
                            t2.b = 255;
                            graphics::Triangle::triangles.push_back(t1);
                            graphics::Triangle::triangles.push_back(t2);
                        }
                    }

                    for (graphics::Light &l : graphics::Light::lights) {
                        l.fillZBuffer(graphics::Triangle::triangles);
                    }
                }
            }
        }

        if (sf::Keyboard::isKeyPressed(sf::Keyboard::W)) {
            cam.moveRelative(0.1, 0, 0);
        }
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::S)) {
            cam.moveRelative(-0.1, 0, 0);
        }
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::A)) {
            cam.moveRelative(0, -0.1, 0);
        }
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::D)) {
            cam.moveRelative(0, 0.1, 0);
        }
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::Space)) {
            cam.moveRelative(0, 0, 0.1);
        }
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::LShift)) {
            cam.moveRelative(0, 0, -0.1);
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

        // MOUSE MOVEMENT
        sf::Vector2i mousePos = sf::Mouse::getPosition();
        mousePos.y /= 3;
        cam.rotate(-(mousePos.x - screenCenter.x) / 400.0, -(mousePos.y - screenCenter.y) / 400.0);
        sf::Mouse::setPosition(screenCenter);



        auto time2 = std::chrono::high_resolution_clock::now();

        // TODO: basic testing for now
        window.clear();
        for (graphics::Line l : floorGrid) {
            l.draw(cam, window);
        }
        for (graphics::Triangle t : graphics::Triangle::triangles) {
            t.draw(cam, window);
        }

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