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