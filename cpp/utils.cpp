#include "utils.h"

float randomFloat() {
    static std::uniform_real_distribution<float> distribution(0.0f, 1.0f);
    static std::mt19937 generator;
    return distribution(generator);
}
float randomFloat(float min, float max) {
    return min + (max - min) * randomFloat();
}