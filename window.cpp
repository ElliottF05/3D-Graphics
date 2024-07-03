#include "window.h"

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
}