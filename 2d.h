#ifndef _2D_H
#define _2D_H

#include <SFML/Graphics/RenderWindow.hpp>

namespace _3d {
    struct Vec3;
}

namespace _2d {

    void drawPoint(sf::RenderWindow& window, const _3d::Vec3& a);
    void drawLine(sf::RenderWindow& window, const _3d::Vec3& a, const _3d::Vec3& b);
    void drawTriangle(sf::RenderWindow& window, const _3d::Vec3&a, const _3d::Vec3&b, const _3d::Vec3&c, float color);
    void drawTriangle(sf::RenderWindow& window, const _3d::Vec3& a, const _3d::Vec3& b, const _3d::Vec3& c);
    
}

#endif