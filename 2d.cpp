#include "2d.h"
#include "3d.h"
#include <SFML/Graphics/Color.hpp>
#include <SFML/Graphics/ConvexShape.hpp>
#include <sfml/Graphics.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <iostream>

void _2d::drawPoint(sf::RenderWindow& window, const _3d::Vec3& a) {
    sf::CircleShape shape(5.f);
    shape.setPosition(sf::Vector2f(a.x - 5, a.y - 5));
    window.draw(shape);
}

void _2d::drawLine(sf::RenderWindow& window, const _3d::Vec3& a, const _3d::Vec3& b) {
    sf::Vertex line[] =
        {
            sf::Vertex(sf::Vector2f(a.x, a.y)),
            sf::Vertex(sf::Vector2f(b.x, b.y))
        };

    window.draw(line, 2, sf::Lines);
}

void _2d::drawTriangle(sf::RenderWindow &window, const _3d::Vec3& a, const _3d::Vec3& b, const _3d::Vec3& c, float color) {
    sf::ConvexShape shape;
    shape.setPointCount(3);
    shape.setPoint(0, sf::Vector2f(a.x, a.y));
    shape.setPoint(1, sf::Vector2f(b.x, b.y));
    shape.setPoint(2, sf::Vector2f(c.x, c.y));
    float colorVal = 255 * color;
    sf::Color fillColor(colorVal, colorVal, colorVal);
    shape.setFillColor(fillColor);
    window.draw(shape);
}