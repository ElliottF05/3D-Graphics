#include <SFML/Graphics/CircleShape.hpp>
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
#include <cmath>
#include <iostream>
#include <chrono>
#include <thread>
#include <sfml/Graphics.hpp>
#include <vector>
#include "2d.h"
#include "3d.h"
#include "window.h"

void spawnTetrehedron(_3d::Camera cam) {
    _3d::Vec3 a = cam.pos;
    _3d::Vec3 b = a + _3d::Vec3(1,0,0);
    _3d::Vec3 c = a + _3d::Vec3(0,1,0);
    _3d::Vec3 d = a + _3d::Vec3(0,0,1);

    _3d::Triangle t1 = *(new _3d::Triangle(a,b,c));
    _3d::Triangle t2 = *(new _3d::Triangle(a,b,d));
    _3d::Triangle t3 = *(new _3d::Triangle(a,c,d));
    _3d::Triangle t4 = *(new _3d::Triangle(b,c,d));
}


int main(int, char**){

    sf::ContextSettings settings;
    settings.antialiasingLevel = 8;

    sf::RenderWindow window(sf::VideoMode(800, 800), "My window", sf::Style::Default, settings);
    window.setFramerateLimit(60);
    //window.setMouseCursorVisible(false);
    sf::Vector2i screenCenter = sf::Vector2i(sf::VideoMode::getDesktopMode().width / 2, sf::VideoMode::getDesktopMode().height / 2);
    sf::Mouse::setPosition(screenCenter);

    //testing some vectors
    _3d::Camera cam = _3d::Camera(_3d::Vec3(0, 0, 2.2), -0, 0, 90);

    std::vector<_3d::Line> floorGrid = std::vector<_3d::Line>();
    for (int i = -5; i <= 5; i++) {
        _3d::Vec3 a = _3d::Vec3(i,5,0);
        _3d::Vec3 b = _3d::Vec3(i, -5, 0);
        _3d::Line c = _3d::Line(a, b);
        _3d::Vec3 d = _3d::Vec3(5,i,0);
        _3d::Vec3 e = _3d::Vec3(-5, i, 0);
        _3d::Line f = _3d::Line(d, e);
        
        floorGrid.push_back(c);
        floorGrid.push_back(f);
    }

    _3d::Vec3 a(10, 1, 2);
    _3d::Vec3 b(5, 1, 2);
    _3d::Vec3 c(5, 3, 5);
    _3d::Vec3 d(10, 10, -2);
    _3d::Triangle t(a, b, c);
    _3d::Triangle t2(a, b, d);
    _3d::Triangle t3(a, c, d);
    _3d::Triangle t4(b,c,d);


    // testing drawing window manually
    wd::PixelArray pixelArray(800, 800);

    // run the program as long as the window is open
    while (window.isOpen()) {

        // check all the window's events that were triggered since the last iteration of the loop
        sf::Event event;
        while (window.pollEvent(event)) {
            // "close requested" event: we close the window
            if (event.type == sf::Event::Closed)
                window.close();

            if (event.type == sf::Event::KeyPressed) {
                if (sf::Keyboard::isKeyPressed(sf::Keyboard::C)) {
                    spawnTetrehedron(cam);
                }
            }

        }
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::Q)) {
            window.close();
            break;
        }
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::Left)) {
            cam.thetaZ += 0.02;
        }
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::Right)) {
            cam.thetaZ -= 0.02;
        }
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::Up)) {
            cam.setThetaY(cam.thetaY + 0.02);
        }
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::Down)) {
            cam.setThetaY(cam.thetaY - 0.02);
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
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::D)) {
            _3d::Vec3 u = cam.getUnitFloorVector();
            _3d::Vec3 v = _3d::Vec3(u.y, -u.x, 0);
            v.scalarMult(0.2);
            cam.pos.add(v);
        }
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::A)) {
            _3d::Vec3 u = cam.getUnitFloorVector();
            _3d::Vec3 v = _3d::Vec3(-u.y, u.x, 0);
            v.scalarMult(0.2);
            cam.pos.add(v);
        }
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::Space)) {
            cam.pos.z += 0.1;
        }
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::LShift)) {
            cam.pos.z -= 0.1;
        }

        sf::Vector2i mousePos = sf::Mouse::getPosition();
        mousePos.y /= 3;
        cam.setThetaZ(cam.thetaZ - (mousePos.x - screenCenter.x) / 400.0);
        cam.setThetaY(cam.thetaY - (mousePos.y - screenCenter.y) / 400.0);
        sf::Mouse::setPosition(screenCenter);

        // clear the window with black color
        window.clear(sf::Color::Black);

        // draw everything here...
        for (_3d::Line a : floorGrid) {
            a.draw(cam, window);
        }

        _3d::Triangle::drawAll(cam, window);

        _3d::Vec3 v1, v2, v3;
        v1 = t.p1;
        v2 = t.p2;
        v3 = t.p3;

        v1.fullyToPlaneCoords(cam);
        v2.fullyToPlaneCoords(cam);
        v3.fullyToPlaneCoords(cam);

        v1.toScreenCoords(cam, window);
        v2.toScreenCoords(cam, window);
        v3.toScreenCoords(cam, window);

        pixelArray.drawTriangle(v1, v2, v3);

        

        sf::Image image;
        image.create(800,800);

        for (int y = 0; y < 800; y++) {
            for (int x = 0; x < 800; x++) {
                int value = pixelArray.getPixel(x, y);
                sf::Color color = sf::Color(value, value, value);
                // std::cout << value << "\n";
                image.setPixel(x, y, color);
            }
        }

        sf::Texture texture;
        texture.create(800, 800);
        texture.update(image);

        sf::Sprite sprite(texture);

        window.draw(sprite);

        // end the current frame
        window.display();

    std::this_thread::sleep_for(std::chrono::milliseconds(16));
    }

    return 0;
}