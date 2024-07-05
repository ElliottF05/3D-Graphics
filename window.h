#pragma once

#include <vector>
#include "3d.h"

namespace wd {

    struct ZBuffer {
        int width, height;
        std::vector<std::vector<float> > data;

        ZBuffer(int width, int height);

        void addPolygon(const _3d::Vec3& a, const _3d::Vec3& b, const _3d::Vec3&c);
    };

    struct PixelArray {
        int width, height;
        std::vector<int> data;

        PixelArray(int width, int height);

        void setPixel(int x, int y, int value);
        int getPixel(int x, int y);

        void clearArray();
        void drawTriangle(const _3d::Vec3& a, const _3d::Vec3&b, const _3d::Vec3& c);
        void drawTriangle(const _3d::Vec3& a, const _3d::Vec3&b, const _3d::Vec3& c, int color);
    };
}