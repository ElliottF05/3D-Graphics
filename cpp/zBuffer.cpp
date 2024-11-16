#include <iostream>

#include "zBuffer.h"

// CONSTRUCTOR
ZBuffer::ZBuffer(int width, int height) {
    this->width = width;
    this->height = height;
    data = std::vector<ZBufferData>(width * height);
}

// METHODS
void ZBuffer::clear() {
    for (int i = 0; i < data.size(); i++) {
        data[i].z = 9999.0f;
    }
}
int ZBuffer::getIndex(int x, int y) {
    // if (x < 0 || x >= width || y < 0 || y >= height) {
    //     std::cout << "PixelArray::getIndex() failed, pixel coordinates out of bounds. INPUTS: x = " << x << 
    //     ", y = " << y << std::endl; 
    //     throw "pixel coordinates out of bounds";
    // }
    return (width * y) + x;
}
void ZBuffer::setPixel(int x, int y, float z) {
    // if (r < 0 || g < 0 || b < 0 || r > 255 || g > 255 || b > 255) {
    //     std::cout << "PixelArray::setPixel() failed, color value out of bounds. INPUTS: r, g, b = " << r << ", " << g << ", " << b << std::endl;
    //     throw "color value out of bounds";
    // }
    int index = getIndex(x, y);
    {
        std::lock_guard<std::mutex> lock(data[index].lock);
        data[index].z = z;
    }
}
void ZBuffer::setPixel(int index, float z) {
    {
        std::lock_guard<std::mutex> lock(data[index].lock);
        data[index].z = z;
    }
}
const std::vector<ZBufferData>& ZBuffer::getData() const {
    return data;
}
int ZBuffer::getWidth() {
    return width;
}
int ZBuffer::getHeight() {
    return height;
}