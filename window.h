#pragma once

#include <vector>
namespace wd {

    struct ZBuffer {
        int width, height;
        std::vector<std::vector<float> > data;

        ZBuffer(int width, int height);
    };
}