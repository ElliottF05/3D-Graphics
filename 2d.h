#ifndef _2D_H
#define _2D_H

#include <SFML/Graphics/RenderWindow.hpp>

namespace _2d {

    struct Vec2;

    void drawPoint(sf::RenderWindow& window, Vec2 a);
    void drawLine(sf::RenderWindow& window, Vec2 a, Vec2 b);

    struct Vec2 {
        float x, y;
        bool inFront;

        Vec2(float x, float y, bool inFront);
        void scalarMult(float k);
        void draw(sf::RenderWindow& window);

    };
    
}

#endif