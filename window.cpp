#include "window.h"
#include <SFML/Config.hpp>
#include <array>
#include <iostream>

using namespace wd;

ZBuffer::ZBuffer(int width, int height) {
    this->width = width;
    this->height = height;

    this->data = std::vector<std::vector<float> >(height);
    for (int i = 0; i < height; i++) {
        this->data.push_back(std::vector<float>(width));
        for (int j = 0; j < width; j++) {
            this->data[i].push_back(0);
        }
    }
}

void ZBuffer::addPolygon(const _3d::Vec3 &a, const _3d::Vec3 &b, const _3d::Vec3 &c) {
    // trying linear interpolation

    // finding leftmost, center, and rightmost points
    _3d::Vec3 left, center, right, temp;
    left = a;
    center = b;
    right = c;

    if (center.x < left.x) {
        temp = left;
        left = center;
        center = temp;
    }
    if (right.x < center.x) {
        temp = center;
        center = right;
        right = temp;
    }
    if (center.x < left.x) {
        temp = left;
        left = center;
        center = temp;
    }

    // TODO: Finish this

}


PixelArray::PixelArray(int width, int height) {
    this->width = width;
    this->height = height;

    this->data = std::vector<int>(width * height * 3);
    for (int i = 0; i < width * height * 3; i++) {
        this->data[i]= 0;
    }
}

void PixelArray::setPixel(int x, int y, int value) {
    int index = 3 * (this->width * y + x);
    this->data[index] = value;
    this->data[index + 1] = value;
    this->data[index + 2] = value;
}

int PixelArray::getPixel(int x, int y) {
    int index = 3 * (this->width * y + x);
    return this->data[index];
}

void PixelArray::clearArray() {
    for (int i = 0; i < this->data.size(); i++) {
        this->data[i] = 0;
    }
}

void PixelArray::drawTriangle(const _3d::Vec3& a, const _3d::Vec3&b, const _3d::Vec3& c) {
    this->drawTriangle(a, b, c, 255);
}

void PixelArray::drawTriangle(const _3d::Vec3 &a, const _3d::Vec3 &b, const _3d::Vec3 &c, int color) {
    // finding leftmost, center, and rightmost points
    _3d::Vec3 left, center, right;
    left = a;
    center = b;
    right = c;

    if (center.x < left.x) {
        std::swap(center, left);
    }
    if (right.x < center.x) {
        std::swap(center, right);
    }
    if (center.x < left.x) {
        std::swap(center, left);
    }

    int leftVal = round(left.x);
    int rightVal = round(right.x);
    int centerVal = round(center.x);
    std::cout << leftVal << ", " << centerVal << ", " << rightVal << "\n";

    float dy1 = (center.y - left.y) / (center.x - left.x);
    float dy2 = (right.y - left.y) / (right.x - left.x);

    for (int i = leftVal; i <= centerVal; i++) {
        if (i >= 800) {
            break;
        }
        if (i < 0) {
            i = 0;
        }
        if (i > centerVal) {
            break;
        }

        int y1 = round(left.y + (i - leftVal) * dy1);
        int y2 = round(left.y + (i - leftVal) * dy2);

        // make y1 less than y2
        if (y1 > y2) {
            std::swap(y1, y2);
        }

        for (int j = y1; j <= y2; j++) {
            if (j >= 800) {
                break;
            }
            if (j < 0) {
                j = 0;
            }

            this->setPixel(i, j, color);
        }
    }

    dy1 = (right.y - center.y) / (right.x - center.x);
    dy2 = (right.y - left.y) / (right.x - left.x);

    for (int i = centerVal; i <= rightVal; i++) {
        if (i >= 800) {
            break;
        }
        if (i < 0) {
            i = 0;
        }
        if (i > rightVal) {
            break;
        }

        int y1 = round(center.y + (i - centerVal) * dy1);
        int y2 = round(left.y + (i - leftVal) * dy2);

        // make y1 less than y2
        if (y1 > y2) {
            std::swap(y1, y2);
        }

        for (int j = y1; j <= y2; j++) {
            if (j >= 800) {
                break;
            }
            if (j < 0) {
                j = 0;
            }

            this->setPixel(i, j, color);
        }
    }
}