#ifndef _2D_H
#define _2D_H

#include <SFML/Graphics/RenderWindow.hpp>

namespace _2d {

    struct Vec2;

    void drawPoint(sf::RenderWindow& window, const Vec2& a);
    void drawLine(sf::RenderWindow& window, const Vec2& a, const Vec2& b);
    void drawTriangle(sf::RenderWindow& window, const Vec2&a, const Vec2&b, const Vec2&c, float color);
    void drawTriangle(sf::RenderWindow& window, const Vec2& a, const Vec2& b, const Vec2& c);

    struct Vec2 {
        float x, y;
        bool inFront;

        Vec2(float x, float y, bool inFront);
        Vec2(float x, float y);
        Vec2();
        void scalarMult(float k);
        void draw(sf::RenderWindow& window);

    };
    
}

#endif