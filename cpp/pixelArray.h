#pragma once

#include <vector>
#include <mutex>

struct PixelArrayData {
    std::mutex lock;
    int r,g,b;
};

class PixelArray {
    private:
        std::vector<PixelArrayData> data;
        int width, height;

        int getIndex(int x, int y);
    public:
        PixelArray(int width, int height);

        void clear();
        void setPixel(int x, int y, int r, int g, int b);
        void setPixel(int index, int r, int g, int b);

        int getWidth();
        int getHeight();
        const std::vector<PixelArrayData>& getData() const;
};