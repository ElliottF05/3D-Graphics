#include "2d.h"
#include <sfml/Graphics.hpp>
#include <SFML/Graphics/RenderWindow.hpp>

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

_2d::Vec2::Vec2(float x, float y, bool inFront) {
    this->x = x;
    this->y = y;
    this->inFront = inFront;
}

void _2d::Vec2::scalarMult(float k) {
    this->x *= k;
    this->y *= k;
}

void _2d::Vec2::draw(sf::RenderWindow& window) {
    drawPoint(window, *this);
}