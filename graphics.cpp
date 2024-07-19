#include "graphics.h"
#include <SFML/Config.hpp>
#include <SFML/Graphics/RenderWindow.hpp>
#include <SFML/Graphics/Sprite.hpp>
#include <SFML/Graphics/Texture.hpp>
#include <iostream>
#include <vector>

using namespace graphics;

//-----------------------------------------------------------------------------------
// IMPLEMENTATION OF "Vec3"

// CONSTRUCTORS
Vec3::Vec3(float x, float y, float z) {
    this->x = x;
    this->y = y;
    this->z = z;
}

Vec3::Vec3() {
    this->x = 0;
    this->y = 0;
    this->z = 0;
};

// OPERATIONS
Vec3 Vec3::operator+(const Vec3& other) const {
    return Vec3(this->x + other.x, this->y + other.y, this->z + other.z);
}
Vec3 Vec3::operator-(const Vec3& other) const {
    return Vec3(this->x - other.x, this->y - other.y, this->z - other.z);
}
Vec3 Vec3::operator*(const float scalar) const {
    return Vec3(this->x * scalar, this->y * scalar, this->z * scalar);
}
Vec3 Vec3::operator/(const float scalar) const {
    float a = 1.0 / scalar;
    return Vec3(this->x * a, this->y * a, this->z * a);
}
Vec3& Vec3::operator+=(const Vec3& vec) {
    this->x += vec.x;
    this->y += vec.y;
    this->z += vec.z;
    return *this;
}
Vec3& Vec3::operator-=(const Vec3& vec) {
    this->x -= vec.x;
    this->y -= vec.y;
    this->z -= vec.z;
    return *this;
}
Vec3& Vec3::operator*=(const float scalar) {
    this->x *= scalar;
    this->y *= scalar;
    this->z *= scalar;
    return *this;
}
Vec3& Vec3::operator/=(const float scalar) {
    this->x /= scalar;
    this->y /= scalar;
    this->z /= scalar;
    return *this;
}
// Operators where Vec3 is right-hand-side
Vec3 operator*(const float scalar, const Vec3& vec) {
    return Vec3(vec.x * scalar, vec.y * scalar, vec.z * scalar);
}
Vec3 operator/(const float scalar, const Vec3&vec) {
    float x = 1.0 / scalar;
    return Vec3(vec.x * x, vec.y * x, vec.z * x);
}

Vec3 Vec3::cross(const Vec3& other) const {
    return Vec3(
        this->y * other.z - this->z * other.y,
        this->z * other.x - this->x * other.z,
        this->x * other.y - this->y * other.x
        );
}
float Vec3::dot(const Vec3& other) const {
    return this->x * other.x + this->y * other.y + this->z * other.z;
}
float Vec3::mag() const {
    return sqrt(this->x * this->x + this->y * this->y + this->z * this->z);
}
void Vec3::normalize() {
    *this /= mag();
}
float Vec3::angleWith(const Vec3 &other) const {
    return acos(this->dot(other) / (this->mag() * other.mag()));
}

// ROTATION
void Vec3::rotateZ(float thetaZ) {
    Vec3 orig = *this;

    this->x = orig.x * cos(thetaZ) - orig.y * sin(thetaZ);
    this->y = orig.x * sin(thetaZ) + orig.y * cos(thetaZ);
}
void Vec3::rotateY(float thetaY) {
    Vec3 orig = *this;

    this->x = orig.x * cos(thetaY) - orig.z * sin(thetaY);
    this->z = orig.x * sin(thetaY) + orig.z * cos(thetaY);
}
void Vec3::rotate(float thetaZ, float thetaY) {
    rotateZ(thetaZ);
    rotateY(thetaY);
}

// TO STRING
std::string Vec3::toString() {
    return std::to_string(this->x) + ", " + std::to_string(this->y) + ", " + std::to_string(this->z);
}



//-----------------------------------------------------------------------------------
// IMPLEMENTATION OF "Point"

// CONSTRUCTORS
Point::Point(Vec3 absolutePos) {
    this->absolutePos = absolutePos;
}
Point::Point() {};

// METHODS
void Point::calculateCameraPos(const Camera &cam) {
    cameraPos = absolutePos - cam.pos;
    cameraPos.rotate(-cam.thetaZ, -cam.thetaY);
    distToCamera = cameraPos.mag();
}
void Point::calculateProjectedPos() {
    // NOTE: x, y, and z now carry different meanings. x = horizontal pos, y = vertical pos
    projectedPos.y = cameraPos.z / cameraPos.x;
    projectedPos.x = cameraPos.y / cameraPos.x;

    if (cameraPos.x > 0) {
        projectedPos.z = 1;
    } else {
        projectedPos.z = -1;
    }
}
void Point::calculateScreenPos(const Camera& cam, const Window &window) {
    screenPos.x = (0.5 * window.width) * (1 - projectedPos.x / cam.maxPlaneCoord);
    screenPos.y = 0.5 * window.height - projectedPos.y / cam.maxPlaneCoord * 0.5 * window.width;
}


//-----------------------------------------------------------------------------------
// IMPLEMENTATION OF "PixelArray"

// CONSTRUCTOR
PixelArray::PixelArray(int width, int height) {
    this->width = width;
    this->height = height;
    this->data = std::vector<int>(width * height * 3, 0);
}

// METHODS
int PixelArray::getIndex(int x, int y) {
    if (x < 0 || x >= width || y < 0 || y >= height) {
        throw "pixel coordinates out of bounds";
    }
    return ((this->width * y) + x) * 3;
}
void PixelArray::setPixel(int x, int y, int color) {
    if (color < 0 || color > 255) {
        throw "color value out of bounds";
    }
    int index = this->getIndex(x, y);
    this->data[index] = color;
    this->data[index+1] = color;
    this->data[index+2] = color;
}
void PixelArray::setPixel(int x, int y, int r, int g, int b) {
    if (r < 0 || g < 0 || b < 0 || r > 255 || g > 255 || b > 255) {
        throw "color value out of bounds";
    }
    int index = this->getIndex(x, y);
    this->data[index] = r;
    this->data[index+1] = g;
    this->data[index+2] = b;
}
int PixelArray::getPixelMonocolor(int x, int y) {
    int index = this->getIndex(x, y);
    return this->data[index];
}
std::vector<int> PixelArray::getPixel(int x, int y) {
    int index = this->getIndex(x, y);
    std::vector<int> v = {this->data[index], this->data[index+1], this->data[index+2]};
    return v;
}
void PixelArray::clear() {
    for (int i = 0; i < data.size(); i++) {
        data[i] = 0;
    }
}


//-----------------------------------------------------------------------------------
// IMPLEMENTATION OF "ZBuffer"

// CONSTRUCTOR
ZBuffer::ZBuffer(int width, int height) {
    this->width = width;
    this->height = height;
    data = std::vector<float>(width * height, 99999);
}

// METHODS
int ZBuffer::getIndex(int x, int y) {
    if (x < 0 || x >= width || y < 0 || y >= height) {
        throw "pixel coordinates out of bounds";
    }
    return ((this->width * y) + x) * 3;
}
void ZBuffer::setDepth(int x, int y, float depth) {
    if (depth < 0) {
        throw "invalid depth";
    }
    int index = getIndex(x, y);
    data[index] = depth;
}
float ZBuffer::getDepth(int x, int y) {
    int index = getIndex(x, y);
    return data[index];
}
void ZBuffer::clear() {
    for (int i = 0; i < data.size(); i++) {
        data[i] = 99999;
    }
}


//-----------------------------------------------------------------------------------
// IMPLEMENTATION OF "Window"

// CONSTRUCTOR
Window::Window(int width, int height, sf::RenderWindow& sfmlWindow)
 : pixelArray(width, height), zBuffer(width, height), sfmlWindow(sfmlWindow) {
    this->width = width;
    this->height = height;
}

void Window::drawPoint(Point& point) {
    if (point.screenPos.x >= 0 && point.screenPos.x < width
    && point.screenPos.y >= 0 && point.screenPos.y < height) {
        pixelArray.setPixel(point.screenPos.x, point.screenPos.y, 255);
    }
}
void Window::drawLine(Line &line) {
    int direction;
    if (line.p2.screenPos.x - line.p1.screenPos.x < 0) {
        direction = -1;
    } else {
        direction = 1;
    }
    float dy = (line.p2.screenPos.y - line.p1.screenPos.y) / (line.p2.screenPos.x - line.p1.screenPos.x);
    int half_dy = abs((int) (dy / 2));

    int startVal = round(line.p1.screenPos.x);
    startVal = std::max(startVal, 0);
    startVal = std::min(startVal, width - 1);
    int endVal = round(line.p2.screenPos.x);
    endVal = std::max(endVal, 0);
    endVal = std::min(endVal, width - 1);

    float y = line.p1.screenPos.y + (startVal - line.p1.screenPos.x) * dy;
    for (int x = startVal; x <= endVal; x += direction) {
        int y_round = round(y);
        for (int i = std::max(0, y_round - half_dy); i <= std::min(height - 1, y_round + half_dy); i++) {
            pixelArray.setPixel(x, i, 255);
        }
        y += dy;
    }

}
void Window::clear() {
    pixelArray.clear();
    zBuffer.clear();
}
void Window::draw() {
    // TODO: WARNING - this is implemntation specific

    // first convert to sfml Uint8 array

    auto t1 = std::chrono::high_resolution_clock::now();
    sf::Uint8 sfpixel[height * width * 4];
    // i gives index in pixelArray, j gives index in sfpixel[]
    for (int i = 0, j = 0; i < height * width * 3; j++) {
        if (j >= height * width * 4) {
            throw "index in sfpixel[] is out of bounds";
        }
        if ((j + 1) % 4 == 0) {
            sfpixel[j] = 255;
        } else {
            sfpixel[j] = static_cast<sf::Uint8>(pixelArray.data[i]);
            i++;
        }
    }

    // second, load to sf::Texture

    auto t2 = std::chrono::high_resolution_clock::now();

    sf::Texture texture;
    texture.create(width, height);
    texture.update(sfpixel);

    // load to sf::Sprinte
    sf::Sprite sprite;
    sprite.setTexture(texture);

    // draw and display
    sfmlWindow.draw(sprite);
    sfmlWindow.display();
    auto t3 = std::chrono::high_resolution_clock::now();
    auto pixelTime = std::chrono::duration_cast<std::chrono::microseconds>(t2 - t1);
    auto spriteTime = std::chrono::duration_cast<std::chrono::microseconds>(t3 - t2);
    // std::cout << "inside graphics - pixel time: " << pixelTime.count() << ", sprite time: " << spriteTime.count() << "\n";
}