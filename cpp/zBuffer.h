#pragma once

#include <vector>
#include <mutex>

struct ZBufferData {
    std::mutex lock;
    float z;
};

class ZBuffer {
    private:
        std::vector<ZBufferData> data;
        int width, height;

        int getIndex(int x, int y);
    public:
        ZBuffer(int width, int height);

        void clear();
        void setPixel(int x, int y, float z);
        void setPixel(int index, float z);
        float getPixel(int x, int y);
        float getPixel(int index);

        int getWidth();
        int getHeight();
        const std::vector<ZBufferData>& getData() const;
        std::vector<ZBufferData>& getMutableData();
};