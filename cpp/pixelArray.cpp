#include <iostream>

#include "pixelArray.h"

// CONSTRUCTOR
PixelArray::PixelArray(int width, int height) {
    this->width = width;
    this->height = height;
    data = std::vector<PixelArrayData>(width * height);
}

// METHODS
void PixelArray::clear() {
    for (int i = 0; i < data.size(); i++) {
        data[i].r = 0;
        data[i].g = 0;
        data[i].b = 0;
    }
}
int PixelArray::getIndex(int x, int y) {
    // if (x < 0 || x >= width || y < 0 || y >= height) {
    //     std::cout << "PixelArray::getIndex() failed, pixel coordinates out of bounds. INPUTS: x = " << x << 
    //     ", y = " << y << std::endl; 
    //     throw "pixel coordinates out of bounds";
    // }
    return (width * y) + x;
}
void PixelArray::setPixel(int x, int y, int r, int g, int b) {
    // if (r < 0 || g < 0 || b < 0 || r > 255 || g > 255 || b > 255) {
    //     std::cout << "PixelArray::setPixel() failed, color value out of bounds. INPUTS: r, g, b = " << r << ", " << g << ", " << b << std::endl;
    //     throw "color value out of bounds";
    // }
    int index = getIndex(x, y);
    PixelArrayData& pixel = data[index];
    {
        std::lock_guard<std::mutex> lock(pixel.lock);
        pixel.r = r;
        pixel.g = g;
        pixel.b = b;
    }
}
void PixelArray::setPixel(int index, int r, int g, int b) {
    {
        std::lock_guard<std::mutex> lock(data[index].lock);
        data[index].r = r;
        data[index].g = g;
        data[index].b = b;
    }
}
const std::vector<PixelArrayData>& PixelArray::getData() const {
    return data;
}
int PixelArray::getWidth() {
    return width;
}
int PixelArray::getHeight() {
    return height;
}