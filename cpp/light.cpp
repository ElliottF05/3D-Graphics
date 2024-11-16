#pragma once

#include "light.h"
#include <cmath>

// CONSTRUCTOR
Light::Light(Vec3 position, float thetaZ, float thetaY, float fov, int r, int g, int b, float luminosity)
    : camera(position, thetaZ, thetaY, fov), r(r), g(g), b(b), luminosity(luminosity), zBuffer(1000, 1000) {
}

// METHODS
float Light::getLightingValue(Vec3 pos) {
    pos -= camera.getPos();
    pos.rotateZ(-camera.getThetaZ());
    pos.rotateY(-camera.getThetaY());

    float depth = pos.x;
    pos.x = pos.y / depth;
    pos.y = pos.z / depth;
    pos.z = depth;

    float xDist = pos.x;
    float yDist = pos.y;

    int width = zBuffer.getWidth();
    int height = zBuffer.getHeight();

    float maxPlaneCoord = tan(camera.getFov() / 2.0f);

    pos.x = (0.5 * width) * (1 - pos.x / maxPlaneCoord);
    pos.y = 0.5 * (height - pos.y / maxPlaneCoord * width);

    if (depth < 0 || pos.x < 0 || pos.x >= width || pos.y < 0 || pos.y >= height) {
        return 0;
    }

    if (depth > zBuffer.getPixel(pos.x, pos.y)) {
        return 0;
    } else {
        zBuffer.setPixel(pos.x, pos.y, depth);
    }

    float invdist = 1.0f / std::sqrt(xDist * xDist + yDist * yDist + depth * depth);

    return luminosity * invdist;
}