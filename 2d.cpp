#include "2d.h"
#include <SFML/Graphics/ConvexShape.hpp>
#include <sfml/Graphics.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <iostream>

void _2d::drawPoint(sf::RenderWindow& window, const Vec2& a) {
    sf::CircleShape shape(5.f);
    shape.setPosition(sf::Vector2f(a.x - 5, a.y - 5));
    window.draw(shape);
}

void _2d::drawLine(sf::RenderWindow& window, const Vec2& a, const Vec2& b) {
    sf::Vertex line[] =
        {
            sf::Vertex(sf::Vector2f(a.x, a.y)),
            sf::Vertex(sf::Vector2f(b.x, b.y))
        };

    window.draw(line, 2, sf::Lines);
}

void _2d::drawTriangle(sf::RenderWindow &window, const Vec2 &a, const Vec2 &b, const Vec2 &c) {
    sf::ConvexShape shape;
    shape.setPointCount(3);
    shape.setPoint(0, sf::Vector2f(a.x, a.y));
    shape.setPoint(1, sf::Vector2f(b.x, b.y));
    shape.setPoint(2, sf::Vector2f(c.x, c.y));
    window.draw(shape);
}

_2d::Vec2::Vec2(float x, float y, bool inFront) {
    this->x = x;
    this->y = y;
    this->inFront = inFront;
}

_2d::Vec2::Vec2(float x, float y) {
    this->x = x;
    this->y = y;
    this->inFront = false;
}

_2d::Vec2::Vec2() {
    this->x = 0;
    this->y = 0;
    this->inFront = false;
}

void _2d::Vec2::scalarMult(float k) {
    this->x *= k;
    this->y *= k;
}

void _2d::Vec2::draw(sf::RenderWindow& window) {
    drawPoint(window, *this);
}