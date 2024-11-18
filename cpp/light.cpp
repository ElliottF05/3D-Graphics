#include "light.h"
#include "zBuffer.h"
#include <algorithm>
#include <cmath>
#include <iostream>

// CONSTRUCTOR
Light::Light(Vec3 position, float thetaZ, float thetaY, float fov, int r, int g, int b, float luminosity)
    : camera(position, thetaZ, thetaY, fov), r(r), g(g), b(b), luminosity(luminosity), zBuffer(4000, 4000) {
}

// METHODS
const ZBuffer& Light::getZBuffer() const {
    return zBuffer;
}
void Light::addVerticesToShadowMap(const std::vector<Vec3>& vertices) {
    for (int i = 0; i < vertices.size(); i += 3) {
        Vec3 v1 = vertices[i];
        Vec3 v2 = vertices[i+1];
        Vec3 v3 = vertices[i+2];

        // 2.0) do not render if normal is pointing towards light - FRONT FACE CULLING
        Vec3 normal = (v3 - v1).cross(v2 - v1);
            normal.normalize();
            Vec3 camToTriangle = v1 - camera.getPos();

            if (normal.dot(camToTriangle) < 0) {
                // std::cout << "triangle pointing towards light" << std::endl;
                continue;
            }

        // 2.1) project vertices

        // 2.1.1) translate vertices to camera space
        v1 -= camera.getPos();
        v2 -= camera.getPos();
        v3 -= camera.getPos();

        // 2.1.2) rotate vertices to camera space
        v1.rotateZ(-camera.getThetaZ());
        v2.rotateZ(-camera.getThetaZ());
        v3.rotateZ(-camera.getThetaZ());

        v1.rotateY(-camera.getThetaY());
        v2.rotateY(-camera.getThetaY());
        v3.rotateY(-camera.getThetaY());

        // std::cout << "v1 after rotate: " << v1.toString() << std::endl;

        // 2.1.3) project vertices to plane space
        float depth = v1.x;
        v1.x = v1.y / depth;
        v1.y = v1.z / depth;
        v1.z = depth;

        depth = v2.x;
        v2.x = v2.y / depth;
        v2.y = v2.z / depth;
        v2.z = depth;

        depth = v3.x;
        v3.x = v3.y / depth;
        v3.y = v3.z / depth;
        v3.z = depth;


        // 2.1.4) scale vertices to screen space
        int width = zBuffer.getWidth();
        int height = zBuffer.getHeight();

        float maxPlaneCoord = tan(camera.getFov() / 2.0f);

        v1.x = (0.5 * width) * (1 - v1.x / maxPlaneCoord);
        v1.y = 0.5 * (height - v1.y / maxPlaneCoord * width);

        v2.x = (0.5 * width) * (1 - v2.x / maxPlaneCoord);
        v2.y = 0.5 * (height - v2.y / maxPlaneCoord * width);

        v3.x = (0.5 * width) * (1 - v3.x / maxPlaneCoord);
        v3.y = 0.5 * (height - v3.y / maxPlaneCoord * width);

        // 2.2) draw triangle
        if (v1.z < 0 || v2.z < 0 || v3.z < 0) {
            continue;
        }

        fillTriangle(v1, v2, v3);
    }
}

void Light::fillTriangle(Vec3& v1, Vec3& v2, Vec3& v3) {
    // std::cout << "filling triangle: " << v1.toString() << ", " << v2.toString() << ", " << v3.toString() << std::endl;
    // depth calculations from https://www.scratchapixel.com/lessons/3d-basic-rendering/rasterization-practical-implementation/visibility-problem-depth-buffer-depth-interpolation.html#:~:text=As%20previously%20mentioned%2C%20the%20correct,z%20%3D%201%20V%200.
    
    // sort vertices by y (v1 has lowest y, v3 has highest y)
    if (v1.y > v2.y) {
        std::swap(v1, v2);
    }
    if (v2.y > v3.y) {
        std::swap(v2, v3);
    }
    if (v1.y > v2.y) {
        std::swap(v1, v2);
    }

    int width = zBuffer.getWidth();
    int height = zBuffer.getHeight();

    // calculate slopes
    float slope1 = (v2.x - v1.x) / (v2.y - v1.y); // slope of line from v1 to v2
    float slope2 = (v3.x - v1.x) / (v3.y - v1.y); // slope of line from v1 to v3
    float slope3 = (v3.x - v2.x) / (v3.y - v2.y); // slope of line from v2 to v3

    if (v1.y == v2.y || v1.y == v3.y || v2.y == v3.y) {
        return;
    }

    // calculate starting and ending x values
    float top = std::max(v1.y, 0.0f);
    float x1 = slope1 * (top - v1.y) + v1.x;
    float x2 = slope2 * (top - v1.y) + v1.x;
    float bottom = std::min(v2.y, height-1.0f);

    // fill top half
    for (float y = top; y < bottom; y++) {
        if (std::isnan(y) || std::isinf(y)) {
            break;
        }
        float left = x1;
        float right = x2;
        
        float q1 = (y - v1.y) / (v2.y - v1.y);
        float invLeftDepth = (1 / v1.z) * (1 - q1) + (1 / v2.z) * q1;

        float q2 = (y - v1.y) / (v3.y - v1.y);
        float invRightDepth = (1 / v1.z) * (1 - q2) + (1 / v3.z) * q2;

        if (left > right) {
            std::swap(left, right);
        }
        left = std::max(0.0f, left);
        right = std::min(width-1.0f, right);
        int yInt = std::round(y);
        for (float x = left; x < right; x++) {
            if (std::isnan(x) || std::isinf(x)) {
                break;
            }
            float q3 = (float) (x - x1) / (x2 - x1);
            float invDepth = invLeftDepth * (1 - q3) + invRightDepth * q3;
            float depth = 1 / invDepth;

            int xInt = std::round(x);

            if (depth < zBuffer.getPixel(xInt, yInt)) {
                zBuffer.setPixel(xInt, yInt, depth);
            }
        }
        x1 += slope1;
        x2 += slope2;
    }

    // fill bottom half
    top = std::max(v2.y, 0.0f);
    x1 = slope3 * (top - v2.y) + v2.x;
    x2 = slope2 * (top - v1.y) + v1.x;
    bottom = std::min(v3.y, height-1.0f);

    for (float y = top; y < bottom; y++) {
        if (std::isnan(y) || std::isinf(y)) {
            break;
        }
        float left = x1;
        float right = x2;

        float q1 = (y - v2.y) / (v3.y - v2.y);
        float invLeftDepth = (1 / v2.z) * (1 - q1) + (1 / v3.z) * q1;

        float q2 = (y - v1.y) / (v3.y - v1.y);
        float invRightDepth = (1 / v1.z) * (1 - q2) + (1 / v3.z) * q2;

        if (left > right) {
            std::swap(left, right);
        }
        left = std::max(0.0f, left);
        right = std::min(width-1.0f, right);
        int yInt = std::round(y);
        for (float x = left; x < right; x++) {
            if (std::isnan(x) || std::isinf(x)) {
                break;
            }
            float q3 = (float) (x - x1) / (x2 - x1);
            float invDepth = invLeftDepth * (1 - q3) + invRightDepth * q3;
            float depth = 1 / invDepth;

            int xInt = std::round(x);

            if (depth < zBuffer.getPixel(xInt, yInt)) {
                zBuffer.setPixel(xInt, yInt, depth);
            }
        }
        x1 += slope3;
        x2 += slope2;
    }
}

void Light::addObjectsToShadowMap(std::vector<Object3D>& objects) {
    for (int i = 0; i < objects.size(); i++) {
        addVerticesToShadowMap(objects[i].getVertices());
    }
}

void Light::resetShadowMap() {
    zBuffer.clear();
}

float Light::getLightingAmount(Vec3& worldPos, const Vec3& cameraPos, Vec3& triangleNormal, const ObjectProperties& properties) {
    // pixelPos starts in world space
    // std::cout << "pixelPos: " << pixelPos.toString() << std::endl;

    Vec3 pixelToLight = camera.getPos() - worldPos;
    Vec3 pixelPos = worldPos;

    pixelPos -= camera.getPos();
    pixelPos.rotateZ(-camera.getThetaZ());
    pixelPos.rotateY(-camera.getThetaY());

    float depth = pixelPos.x;
    pixelPos.x = pixelPos.y / depth;
    pixelPos.y = pixelPos.z / depth;
    pixelPos.z = depth;

    float xDist = pixelPos.x;
    float yDist = pixelPos.y;

    int width = zBuffer.getWidth();
    int height = zBuffer.getHeight();

    float maxPlaneCoord = tan(camera.getFov() / 2.0f);

    pixelPos.x = (0.5 * width) * (1 - pixelPos.x / maxPlaneCoord);
    pixelPos.y = 0.5 * (height - pixelPos.y / maxPlaneCoord * width);

    if (depth < 0 || pixelPos.x < 0 || pixelPos.x >= width || pixelPos.y < 0 || pixelPos.y >= height) {
        // std::cout << "pixel not in light's view" << std::endl;
        return 0;
    }

    float shadowAmount = 0;
    int samples = 0;
    int filterRadius = 1;
    float bias = 0.01f;
    for (int dy = -filterRadius; dy <= filterRadius; dy++) {
        for (int dx = -filterRadius; dx <= filterRadius; dx++) {
            Vec3 samplePos = pixelPos + Vec3(dx, dy, 0);
            if (samplePos.x < 0 || samplePos.x >= width || samplePos.y < 0 || samplePos.y >= height) {
                continue;
            }
            if (depth + bias < zBuffer.getPixel(samplePos.x, samplePos.y)) {
                shadowAmount += 1;
            }
            samples++;
        }
    }

    // if (depth > zBuffer.getPixel(pixelPos.x, pixelPos.y)) {
    //     // std::cout << "pixel is occluded, depth: " << depth << ", zBuffer depth: " << zBuffer.getPixel(pixelPos.x, pixelPos.y) << std::endl;
    //     return 0;
    // }


    if (shadowAmount == 0) { // in shadow
        return 0;
    }

    float ambientLight = properties.k_a;

    pixelToLight.normalize();
    float angleMultiplier = pixelToLight.dot(triangleNormal);

    if (angleMultiplier < 0) { // facing away from light
        return ambientLight;
    }

    shadowAmount /= samples;
    float invDist = 1.0f / std::sqrt(xDist * xDist + yDist * yDist + depth * depth);

    float diffuseLight = properties.k_d * angleMultiplier;

    float specularLight = 0;
    if (properties.k_s > 0) {
        Vec3 R = 2 * pixelToLight.dot(triangleNormal) * triangleNormal - pixelToLight;
        Vec3 V = cameraPos - worldPos;
        V.normalize();
        // std::cout << "cameraPos: " << cameraPos.toString() << ", worldPos: " << worldPos.toString() << std::endl;
        float RdotV = R.dot(V);
        if (RdotV >= 0) {
            specularLight = properties.k_s * std::pow(RdotV, properties.shininess);
        }
    }

    return ambientLight + luminosity * invDist * shadowAmount * (diffuseLight + specularLight);
}