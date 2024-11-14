#include "game.h"
#include "object3D.h"
#include <algorithm>
#include <iostream>

// CONSTRUCTOR
Game::Game() : pixelArray(500, 500), camera(Vec3(0,0,0), 0, 0, M_PI/2.0f) {
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
    camera = Camera(Vec3(-0.5f,-0.5f,1.5f), 0, 0, M_PI/2.0f);
}

void Game::render() {
    // std::cout << "game.cpp: render() called" << std::endl;

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

            float maxPlaneCoord = tan(camera.getFov() / 2);
            v1.x = (0.5 * width) * (1 - v1.x / maxPlaneCoord) - 0.5;
            v1.y = 0.5 * (height - v1.y / maxPlaneCoord * width) - 0.5;

            v2.x = (0.5 * width) * (1 - v2.x / maxPlaneCoord) - 0.5;
            v2.y = 0.5 * (height - v2.y / maxPlaneCoord * width) - 0.5;

            v3.x = (0.5 * width) * (1 - v3.x / maxPlaneCoord) - 0.5;
            v3.y = 0.5 * (height - v3.y / maxPlaneCoord * width) - 0.5;

            // 2.2) draw triangle
            if (v1.z < 0 || v2.z < 0 || v3.z < 0) {
                continue;
            }
            fillTriangle(v1, v2, v3, obj.getR(), obj.getG(), obj.getB());

        }
    }
}

void Game::fillTriangle(Vec3& v1, Vec3& v2, Vec3& v3, int r, int g, int b) {
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
    float slope1 = (v2.x - v1.x) / (v2.y - v1.y);
    float slope2 = (v3.x - v1.x) / (v3.y - v1.y);
    float slope3 = (v3.x - v2.x) / (v3.y - v2.y);

    // calculate starting and ending x values
    int top = std::max(v1.y, 0.0f);
    float x1 = slope1 * (top - v1.y) + v1.x;
    float x2 = slope2 * (top - v1.y) + v1.x;
    int bottom = std::min(v2.y, 500.0f);

    // fill top half
    for (int y = top; y < bottom; y++) {
        int left = x1;
        int right = x2;
        if (left > right) {
            std::swap(left, right);
        }
        left = std::max(0, left);
        right = std::min(500, right);
        for (int x = left; x < right; x++) {
            pixelArray.setPixel(x, y, r, g, b);
        }
        x1 += slope1;
        x2 += slope2;
    }

    // fill bottom half

    top = std::max(v2.y, 0.0f);
    x1 = slope1 * (top - v2.y) + v2.x;
    x2 = slope2 * (top - v1.y) + v1.x;
    bottom = std::min(v3.y, 500.0f);

    for (int y = top; y < bottom; y++) {
        int left = x1;
        int right = x2;
        if (left > right) {
            std::swap(left, right);
        }
        left = std::max(0, left);
        right = std::min(500, right);
        for (int x = left; x < right; x++) {
            pixelArray.setPixel(x, y, r, g, b);
        }
        x1 += slope3;
        x2 += slope2;
    }

}

uint8_t* Game::exportImageBuffer() {
    // std::cout << "game.cpp: exportImageBuffer() called" << std::endl;
    for (int i = 0; i < 500 * 500; i++) {
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