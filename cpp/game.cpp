#include "game.h"
#include "object3D.h"
#include <algorithm>
#include <chrono>
#include <iostream>

// CONSTRUCTOR
Game::Game() : pixelArray(500, 500), zBuffer(500, 500), camera(Vec3(0,0,0), 0, 0, M_PI/2.0f) {
    imageBuffer = new uint8_t[500 * 500 * 4];
}

void Game::setupScene() {
    std::cout << "game.cpp: setupScene() called" << std::endl;

    // create objects
    objects.emplace_back(std::vector<Vec3>{
        Vec3(10,0,1),
        Vec3(10,4,2),
        Vec3(10,0,5)
    }, 255, 0, 0, 0, false);

    // make grid
    std::vector<Vec3> darkGrey;
    std::vector<Vec3> lightGrey;
    int gridRadius = 2;
    for (int i = -gridRadius; i < gridRadius; i++) {
        for (int j = -gridRadius; j < gridRadius; j++) {
            std::vector<Vec3>& addingTo = ((i + j) % 2 == 0) ? lightGrey : darkGrey;
            addingTo.emplace_back(i,j,0);
            addingTo.emplace_back(i+1,j,0);
            addingTo.emplace_back(i+1,j+1,0);
            addingTo.emplace_back(i,j,0);
            addingTo.emplace_back(i,j+1,0);
            addingTo.emplace_back(i+1,j+1,0);
        }
    }
    std::cout << "darkGrey size: " << darkGrey.size() << std::endl;
    objects.emplace_back(darkGrey, 140, 140, 140, 0, false);
    objects.emplace_back(lightGrey, 200, 200, 200, 0, false);

    std::vector<Vec3> testObj;
    testObj.emplace_back(0,0,0.5f);
    testObj.emplace_back(0,1,0.5f);
    testObj.emplace_back(1,1,0.5f);
    testObj.emplace_back(0,0,0.5f);
    testObj.emplace_back(1,0,0.5f);
    testObj.emplace_back(1,1,0.5f);
    objects.emplace_back(testObj, 220, 220, 220, 0, false);

    // create camera
    camera = Camera(Vec3(-0.5f,-0.5f,1.5f), 0.0111, 0.0111, M_PI/2.0f);
}

void Game::render() {
    // CURRENTLY TAKING 2.2 to 2.4 ms on average

    // std::cout << "game.cpp: render() called" << std::endl;
    // auto startTime = std::chrono::high_resolution_clock::now();
    // auto fillTriangleTime = startTime - startTime;

    // 1) clear screen
    pixelArray.clear();

    // 2) render objects
    for (const Object3D& obj : objects) {
        const std::vector<Vec3> vertices = obj.getVertices(); // THIS IS A COPY
        for (int i = 0; i < vertices.size(); i += 3) {
            Vec3 v1 = vertices[i];
            Vec3 v2 = vertices[i+1];
            Vec3 v3 = vertices[i+2];

            // 2.1) project vertices

            // 2.1.1) translate vertices to camera space
            v1 = v1 - camera.getPos();
            v2 = v2 - camera.getPos();
            v3 = v3 - camera.getPos();

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

            // std::cout << "v1 after projection: " << v1.toString() << std::endl;

            // 2.1.4) scale vertices to screen space
            int width = pixelArray.getWidth();
            int height = pixelArray.getHeight();

            float maxPlaneCoord = tan(camera.getFov() / 2.0f);
            maxPlaneCoord = 1;

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

            // auto preFillTriangle = std::chrono::high_resolution_clock::now();
            fillTriangle(v1, v2, v3, obj.getR(), obj.getG(), obj.getB());
            // auto afterFillTriangle = std::chrono::high_resolution_clock::now();
            // fillTriangleTime += afterFillTriangle - preFillTriangle;


        }
    }
    // auto endTime = std::chrono::high_resolution_clock::now();
    // auto totalDuration = std::chrono::duration_cast<std::chrono::microseconds>(endTime - startTime);
    // auto fillTriangleDuration = std::chrono::duration_cast<std::chrono::microseconds>(fillTriangleTime);
    // std::cout << "total frame time: " << totalDuration.count() << std::endl;
    // std::cout << "total triangle fill time: " << fillTriangleDuration.count() << std::endl;
}

void Game::fillTriangle(Vec3& v1, Vec3& v2, Vec3& v3, int r, int g, int b) {
    // depth calculations from https://www.scratchapixel.com/lessons/3d-basic-rendering/rasterization-practical-implementation/visibility-problem-depth-buffer-depth-interpolation.html#:~:text=As%20previously%20mentioned%2C%20the%20correct,z%20%3D%201%20V%200.
    
    // std::cout << "game.cpp: fillTriangle() called" << std::endl;
    // std::cout << "v1.y = " << v1.y << ", v2.y = " << v2.y << ", v3.y = " << v3.y << std::endl;
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

    // calculate slopes
    float slope1 = (v2.x - v1.x) / (v2.y - v1.y); // slope of line from v1 to v2
    float slope2 = (v3.x - v1.x) / (v3.y - v1.y); // slope of line from v1 to v3
    float slope3 = (v3.x - v2.x) / (v3.y - v2.y); // slope of line from v2 to v3

    // calculate starting and ending x values
    float top = std::max(v1.y, 0.0f);
    float x1 = slope1 * (top - v1.y) + v1.x;
    float x2 = slope2 * (top - v1.y) + v1.x;
    float bottom = std::min(v2.y, 500.0f);

    // fill top half
    for (int y = round(top); y < round(bottom); y++) {
        int left = x1;
        int right = x2;
        
        float q1 = (y - v1.y) / (v2.y - v1.y);
        float invLeftDepth = (1 / v1.z) * (1 - q1) + (1 / v2.z) * q1;

        float q2 = (y - v1.y) / (v3.y - v1.y);
        float invRightDepth = (1 / v1.z) * (1 - q2) + (1 / v3.z) * q2;

        if (left > right) {
            std::swap(left, right);
        }
        left = std::max(0, left);
        right = std::min(500, right);
        for (int x = left; x < right; x++) {
            float q3 = (float) (x - x1) / (x2 - x1);
            float invDepth = invLeftDepth * (1 - q3) + invRightDepth * q3;
            float depth = 1 / invDepth;

            depth = depth - floor(depth);
            pixelArray.setPixel(x, y, depth * r, depth * g, depth * b);
        }
        x1 += slope1;
        x2 += slope2;
    }

    // fill bottom half
    top = std::max(v2.y, 0.0f);
    x1 = slope3 * (top - v2.y) + v2.x;
    x2 = slope2 * (top - v1.y) + v1.x;
    bottom = std::min(v3.y, 500.0f);

    for (int y = round(top); y < round(bottom); y++) {
        int left = x1;
        int right = x2;

        float q1 = (y - v2.y) / (v3.y - v2.y);
        float invLeftDepth = (1 / v2.z) * (1 - q1) + (1 / v3.z) * q1;

        float q2 = (y - v1.y) / (v3.y - v1.y);
        float invRightDepth = (1 / v1.z) * (1 - q2) + (1 / v3.z) * q2;

        if (left > right) {
            std::swap(left, right);
        }
        left = std::max(0, left);
        right = std::min(500, right);
        for (int x = left; x < right; x++) {
            float q3 = (float) (x - x1) / (x2 - x1);
            float invDepth = invLeftDepth * (1 - q3) + invRightDepth * q3;
            float depth = 1 / invDepth;

            depth = depth - floor(depth);
            pixelArray.setPixel(x, y, depth * r, depth * g, depth * b);
        }
        x1 += slope3;
        x2 += slope2;
    }

}

uint8_t* Game::exportImageBuffer() {
    // std::cout << "game.cpp: exportImageBuffer() called" << std::endl;
    for (int i = 0; i < pixelArray.getWidth() * pixelArray.getHeight(); i++) {
        const PixelArrayData& data = pixelArray.getData()[i];
        imageBuffer[4*i] = data.r;
        imageBuffer[4*i+1] = data.g;
        imageBuffer[4*i+2] = data.b;
        imageBuffer[4*i+3] = 255;
    }
    return &imageBuffer[0];
}

void Game::userCameraInput(float forwardMovement, float sidewaysMovement, float verticalMovement, float rotateZ, float rotateY, float otherInputCode) {
    // std::cout << "game.cpp: userCameraInput() called" << std::endl;

    Vec3 pos = camera.getPos();

    // std::cout << "camera pos1: " << pos.toString() << std::endl;

    Vec3 movement = Vec3(forwardMovement,sidewaysMovement,verticalMovement);
    movement.rotateZ(camera.getThetaZ());
    pos = pos + movement;
    camera.setPos(pos);

    // std::cout << "camera pos2: " << camera.getPos().toString() << std::endl;

    camera.setThetaY(camera.getThetaY() + rotateY);
    camera.setThetaZ(camera.getThetaZ() + rotateZ);

    // std::cout << "camera pos3: " << camera.getPos().toString() << std::endl;
}