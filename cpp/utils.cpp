#include "utils.h"
#include "pixelArray.h"

float randomFloat() {
    static std::uniform_real_distribution<float> distribution(0.0f, 1.0f);
    static std::mt19937 generator;
    return distribution(generator);
}
float randomFloat(float min, float max) {
    return min + (max - min) * randomFloat();
}
Vec3 randomInUnitDisk() {
    while (true) {
        Vec3 v = Vec3(randomFloat(-1,1), randomFloat(-1,1), 0);
        if (v.lengthSquared() < 1) {
            return v;
        }
    }
}

float degreesToRadians(float degrees) {
    return degrees * M_PI / 180;
}