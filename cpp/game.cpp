#include "game.h"
#include "camera.h"
#include "object3D.h"
#include "threads.h"
#include "zBuffer.h"
#include <algorithm>
#include <chrono>
#include <cmath>
#include <functional>
#include <iostream>

// CONSTRUCTOR
Game::Game() : pixelArray(500, 500), zBuffer(500, 500), camera(Vec3(0,0,0), 0, 0, M_PI/2.0f) {
    imageBuffer = new uint8_t[500 * 500 * 4];
}

void Game::setupScene() {
    std::cout << "game.cpp: setupScene() called" << std::endl;

    // make grid
    // std::vector<Vec3> darkGrey;
    // std::vector<Vec3> lightGrey;
    // int gridRadius = 5;
    // for (int i = -gridRadius; i < gridRadius; i++) {
    //     for (int j = -gridRadius; j < gridRadius; j++) {
    //         std::vector<Vec3>& addingTo = ((i + j) % 2 == 0) ? lightGrey : darkGrey;
    //         addingTo.emplace_back(i,j,0);
    //         addingTo.emplace_back(i+1,j+1,0);
    //         addingTo.emplace_back(i+1,j,0);
    //         addingTo.emplace_back(i,j,0);
    //         addingTo.emplace_back(i,j+1,0);
    //         addingTo.emplace_back(i+1,j+1,0);
    //     }
    // }
    // objects.emplace_back(darkGrey, 140, 140, 140, 1.0f, 1.0f, 0.2f, 20, true);
    // objects.emplace_back(lightGrey, 200, 200, 200, 1.0f, 1.0f, 0.2f, 20, true);

    // std::vector<Vec3> testObjVertices;
    // float radius = 0.5f;
    // int iterations = 100;

    // Vec3 center = Vec3(1,1,radius);

    // std::vector<Vec3> prev;
    // for (int i = 0; i < iterations; i++) {
    //     prev.push_back(center + Vec3(0,0,radius));
    // }
    // std::vector<Vec3> curr;
    // for (int i = 1; i < iterations; i++) {
    //     float z = radius * std::sin(M_PI / 2.0f - i * M_PI / iterations);
    //     for (int j = 0; j < iterations; j++) {
    //         float x = radius * std::cos(j * 2.0f * M_PI / iterations) * std::cos(M_PI / 2.0f - i * M_PI / iterations);
    //         float y = radius * std::sin(j * 2.0f * M_PI / iterations) * std::cos(M_PI / 2.0f - i * M_PI / iterations);
    //         curr.push_back(center + Vec3(x,y,z));
    //     }
    //     for (int i = 0; i < iterations; i++) {
    //         testObjVertices.push_back(prev[i]);
    //         testObjVertices.push_back(curr[(i+1) % iterations]);
    //         testObjVertices.push_back(curr[i]);
    //         testObjVertices.push_back(prev[i]);
    //         testObjVertices.push_back(prev[(i+1) % iterations]);
    //         testObjVertices.push_back(curr[(i+1) % iterations]);
    //     }
    //     prev = curr;
    //     curr.clear();
    // }

    // Object3D testObj = Object3D(testObjVertices, 220, 220, 220, 1.0f, 1.0f, 0.2f, 20, true);
    // objects.push_back(testObj);

    // create light
    lights.emplace_back(Vec3(-10,0,10), 0, -M_PI/4.0f, M_PI/4.0f, 255, 255, 255, 12);
    lights[0].resetShadowMap();
    lights[0].addObjectsToShadowMap(objects);

    // create camera
    camera = Camera(Vec3(-0.511111f,-0.511111f,1.511111f), 0.0111111, 0.0111111, M_PI/2.0f);

    std::cout << "game.cpp: setupScene() finished" << std::endl;
}

void Game::render() {
    // before zBuffer (only simple color fill per pixel) 2.2 to 2.4 ms on average
    // after zBuffer: 5-6ms on average
    // after shadowMapping (no filtering): 17-18ms on average
    // with shadowmap filtering, float math (small impact), and 16x the shadow map res: 45ms on average
    // with Phong reflections: 45ms on average

    // no parallelization: 115ms, cpu usage 99%
    // each (projectTriangle + fillTriangle()) parallelized: 30ms, cpu usage 480%
    // projectTriangle sequential, fillTriangle parallelized: 31ms

    // std::cout << "game.cpp: render() called" << std::endl;
    auto startTime = std::chrono::high_resolution_clock::now();
    // auto fillTriangleTime = startTime - startTime;

    // 1) clear screen
    pixelArray.clear();
    zBuffer.clear();

    for (Object3D& obj : objects) {
        std::vector<Vec3>& vertices = obj.getMutableVertices();
        for (int i = 0; i < vertices.size(); i += 3) {
            Vec3 v1 = vertices[i];
            Vec3 v2 = vertices[i+1];
            Vec3 v3 = vertices[i+2];
            threadPool.addTask([this, v1, v2, v3, &obj]() mutable {
                projectTriangle(v1, v2, v3, obj.getProperties());
            });
        }
    }

    threadPool.waitUntilTasksFinished();

    // 2) render objects
    // 2.0) set up parallelization
    // std::vector<Vec3> vertices;
    // std::vector<const ObjectProperties*> properties;
    // int numVertices = 0;
    // for (const Object3D& obj : objects) {
    //     numVertices += obj.getVertices().size();
    // }
    // vertices.reserve(numVertices);
    // properties.reserve(numVertices);

    // for (const Object3D& obj : objects) {
    //     for (const Vec3& vertex : obj.getVertices()) {
    //         vertices.push_back(vertex);
    //         properties.push_back(&obj.getProperties());
    //     }
    // }

    // // 2.1) project vertices
    // const int BATCH_SIZE = 1;
    // for (int start = 0; start < vertices.size(); start += BATCH_SIZE * 3) {
    //     int end = std::min(start + BATCH_SIZE * 3, static_cast<int>(vertices.size()));
    //     threadPool.addTask([this, start, end, &vertices, &properties] {
    //         projectTriangleBatch(vertices, properties, start, end);
    //     });
    // }

    // threadPool.waitUntilTasksFinished();

    auto endTime = std::chrono::high_resolution_clock::now();
    auto totalDuration = std::chrono::duration_cast<std::chrono::milliseconds>(endTime - startTime);
    // auto fillTriangleDuration = std::chrono::duration_cast<std::chrono::microseconds>(fillTriangleTime);
    std::cout << "total frame time: " << totalDuration.count() << std::endl;
    // std::cout << "total triangle fill time: " << fillTriangleDuration.count() << std::endl;
}

void Game::projectTriangleBatch(std::vector<Vec3>& vertices, std::vector<const ObjectProperties*>& properties, int start, int end) {
    for (int i = start; i < end; i += 3) {
        projectTriangle(vertices[i], vertices[i+1], vertices[i+2], *properties[i]);
    }
}

void Game::projectTriangle(Vec3& v1, Vec3& v2, Vec3& v3, const ObjectProperties& properties) {

    // do not render if normal is pointing away from cam - BACK FACE CULLING
    Vec3 normal = (v3 - v1).cross(v2 - v1);
    normal.normalize();
    Vec3 camToTriangle = v1 - camera.getPos();

    if (normal.dot(camToTriangle) > 0) {
        return;
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
    int width = pixelArray.getWidth();
    int height = pixelArray.getHeight();

    float maxPlaneCoord = tan(camera.getFov() / 2.0f);

    v1.x = (0.5 * width) * (1 - v1.x / maxPlaneCoord);
    v1.y = 0.5 * (height - v1.y / maxPlaneCoord * width);

    v2.x = (0.5 * width) * (1 - v2.x / maxPlaneCoord);
    v2.y = 0.5 * (height - v2.y / maxPlaneCoord * width);

    v3.x = (0.5 * width) * (1 - v3.x / maxPlaneCoord);
    v3.y = 0.5 * (height - v3.y / maxPlaneCoord * width);

    // 2.2) draw triangle
    if (v1.z < 0 || v2.z < 0 || v3.z < 0) {
        return;
    }

    // auto preFillTriangle = std::chrono::high_resolution_clock::now();
    fillTriangle(v1, v2, v3, properties, normal);
    // auto afterFillTriangle = std::chrono::high_resolution_clock::now();
    // fillTriangleTime += afterFillTriangle - preFillTriangle;

    // threadPool.addTask([this, v1, v2, v3, &properties, normal] {
    //     fillTriangleParallel(v1, v2, v3, properties, normal);
    // });
}

void Game::fillTriangle(Vec3& v1, Vec3& v2, Vec3& v3, const ObjectProperties& properties, Vec3& normal) {
    // depth calculations from https://www.scratchapixel.com/lessons/3d-basic-rendering/rasterization-practical-implementation/visibility-problem-depth-buffer-depth-interpolation.html#:~:text=As%20previously%20mentioned%2C%20the%20correct,z%20%3D%201%20V%200.

    // std::cout << "v1: " << v1.toString() << " v2: " << v2.toString() << " v3: " << v3.toString() << std::endl;

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

    int height = pixelArray.getHeight();
    int width = pixelArray.getWidth();

    // calculate slopes
    float slope1 = (v2.x - v1.x) / (v2.y - v1.y); // slope of line from v1 to v2
    float slope2 = (v3.x - v1.x) / (v3.y - v1.y); // slope of line from v1 to v3
    float slope3 = (v3.x - v2.x) / (v3.y - v2.y); // slope of line from v2 to v3

    if (v1.y == v2.y || v1.y == v3.y || v2.y == v3.y) {
        return;
    }

    // calculate starting and ending x values
    int top = std::max(static_cast<int>(std::ceil(v1.y)), 0);
    float x1 = slope1 * (top - v1.y) + v1.x;
    float x2 = slope2 * (top - v1.y) + v1.x;
    int bottom = std::min(static_cast<int>(std::floor(v2.y)), height-1);

    // fill top half
    for (int y = top; y <= bottom; y++) {

        int left, right;
        if (x1 < x2) {
            left = std::max(static_cast<int>(std::ceil(x1)), 0);
            right = std::min(static_cast<int>(std::floor(x2)), width-1);
        } else {
            left = std::max(static_cast<int>(std::ceil(x2)), 0);
            right = std::min(static_cast<int>(std::floor(x1)), width-1);
        }
        
        float q1 = (y - v1.y) / (v2.y - v1.y);
        float invLeftDepth = (1 / v1.z) * (1 - q1) + (1 / v2.z) * q1;

        float q2 = (y - v1.y) / (v3.y - v1.y);
        float invRightDepth = (1 / v1.z) * (1 - q2) + (1 / v3.z) * q2;

        // PARALLELIZATION OPTION HERE
        // fillHorizontalLine(y, x1, x2, invLeftDepth, invRightDepth, properties, normal);
        // threadPool.addTask([this, y, x1, x2, invLeftDepth, invRightDepth, properties, normal] {
        //     fillHorizontalLine(y, x1, x2, invLeftDepth, invRightDepth, properties, normal);
        // });

        int baseIndex = width * y;

        for (int x = left; x <= right; x++) {

            float q3 = (float) (x - x1) / (x2 - x1);
            float invDepth = invLeftDepth * (1 - q3) + invRightDepth * q3;
            float depth = 1 / invDepth;

            int index = baseIndex + x;

            if (depth < zBuffer.getPixel(index)) {
                zBuffer.setPixel(index, depth);

                Vec3 worldPos = getPlaneCoords(x, y) * depth;
                worldPos.rotateY(camera.getThetaY());
                worldPos.rotateZ(camera.getThetaZ());
                worldPos += camera.getPos();

                float lightingAmount = lights[0].getLightingAmount(worldPos, camera.getPos(), normal, properties);
                lightingAmount = std::max(0.2f, lightingAmount);
                int lightingR = std::min(255, static_cast<int>(properties.r * lightingAmount));
                int lightingG = std::min(255, static_cast<int>(properties.g * lightingAmount));
                int lightingB = std::min(255, static_cast<int>(properties.b * lightingAmount));

                pixelArray.setPixel(index, lightingR, lightingG, lightingB);
            }
        }
        x1 += slope1;
        x2 += slope2;
    }

    // fill bottom half
    top = std::max(static_cast<int>(std::ceil(v2.y)), 0);
    x1 = slope3 * (top - v2.y) + v2.x;
    x2 = slope2 * (top - v1.y) + v1.x;
    bottom = std::min(static_cast<int>(std::floor(v3.y)), height-1);

    for (int y = top; y <= bottom; y++) {

        int left, right;
        if (x1 < x2) {
            left = std::max(static_cast<int>(std::ceil(x1)), 0);
            right = std::min(static_cast<int>(std::floor(x2)), width-1);
        } else {
            left = std::max(static_cast<int>(std::ceil(x2)), 0);
            right = std::min(static_cast<int>(std::floor(x1)), width-1);
        }

        float q1 = (y - v2.y) / (v3.y - v2.y);
        float invLeftDepth = (1 / v2.z) * (1 - q1) + (1 / v3.z) * q1;

        float q2 = (y - v1.y) / (v3.y - v1.y);
        float invRightDepth = (1 / v1.z) * (1 - q2) + (1 / v3.z) * q2;

        // PARALLELIZATION OPTION HERE
        // fillHorizontalLine(y, x1, x2, invLeftDepth, invRightDepth, properties, normal);
        // threadPool.addTask([this, y, x1, x2, invLeftDepth, invRightDepth, properties, normal] {
        //     fillHorizontalLine(y, x1, x2, invLeftDepth, invRightDepth, properties, normal);
        // });

        int baseIndex = width * y;

        for (int x = left; x <= right; x++) {

            float q3 = (float) (x - x1) / (x2 - x1);
            float invDepth = invLeftDepth * (1 - q3) + invRightDepth * q3;
            float depth = 1 / invDepth;

            int index = baseIndex + x;

            if (depth < zBuffer.getPixel(index)) {
                zBuffer.setPixel(index, depth);

                Vec3 worldPos = getPlaneCoords(x, y) * depth;
                worldPos.rotateY(camera.getThetaY());
                worldPos.rotateZ(camera.getThetaZ());
                worldPos += camera.getPos();

                float lightingAmount = lights[0].getLightingAmount(worldPos, camera.getPos(), normal, properties);
                lightingAmount = std::max(0.2f, lightingAmount);
                int lightingR = std::min(255, static_cast<int>(properties.r * lightingAmount));
                int lightingG = std::min(255, static_cast<int>(properties.g * lightingAmount));
                int lightingB = std::min(255, static_cast<int>(properties.b * lightingAmount));

                pixelArray.setPixel(index, lightingR, lightingG, lightingB);
            }
        }
        x1 += slope3;
        x2 += slope2;
    }
}

void Game::fillTriangleOwned(Vec3 v1, Vec3 v2, Vec3 v3, const ObjectProperties& properties, Vec3 normal) {
    fillTriangle(v1, v2, v3, properties, normal);
}

void Game::fillHorizontalLine(int y, float x1, float x2, float invLeftDepth, float invRightDepth, const ObjectProperties& properties, Vec3 normal) {
    int left, right;
    int width = pixelArray.getWidth();
    if (x1 < x2) {
        left = std::max(static_cast<int>(std::ceil(x1)), 0);
        right = std::min(static_cast<int>(std::floor(x2)), width-1);
    } else {
        left = std::max(static_cast<int>(std::ceil(x2)), 0);
        right = std::min(static_cast<int>(std::floor(x1)), width-1);
    }

    int baseIndex = width * y;

    for (int x = left; x <= right; x++) {

        float q3 = (float) (x - x1) / (x2 - x1);
        float invDepth = invLeftDepth * (1 - q3) + invRightDepth * q3;
        float depth = 1 / invDepth;

        int index = baseIndex + x;

        if (depth < zBuffer.getPixel(index)) {
            zBuffer.setPixel(index, depth);

            Vec3 worldPos = getPlaneCoords(x, y) * depth;
            worldPos.rotateY(camera.getThetaY());
            worldPos.rotateZ(camera.getThetaZ());
            worldPos += camera.getPos();

            float lightingAmount = lights[0].getLightingAmount(worldPos, camera.getPos(), normal, properties);
            lightingAmount = std::max(0.2f, lightingAmount);
            int lightingR = std::min(255, static_cast<int>(properties.r * lightingAmount));
            int lightingG = std::min(255, static_cast<int>(properties.g * lightingAmount));
            int lightingB = std::min(255, static_cast<int>(properties.b * lightingAmount));

            pixelArray.setPixel(index, lightingR, lightingG, lightingB);
        }
    }
}

Vec3 Game::getPlaneCoords(int xPixel, int yPixel) {
    int width = pixelArray.getWidth();
    int height = pixelArray.getHeight();
    float maxPlaneCoord = tan(camera.getFov() / 2.0f);

    // v1.x = (0.5 * width) * (1 - v1.x / maxPlaneCoord);
    // v1.y = 0.5 * (height - v1.y / maxPlaneCoord * width);

    return Vec3(
        1,
        -((xPixel * 2.0f / width - 1) * maxPlaneCoord),
        -((yPixel * 2.0f - height) / width * maxPlaneCoord)
    );
}
Vec3 Game::getPlaneCoords(float xPixel, float yPixel) {
    int width = pixelArray.getWidth();
    int height = pixelArray.getHeight();
    float maxPlaneCoord = tan(camera.getFov() / 2.0f);

    // v1.x = (0.5 * width) * (1 - v1.x / maxPlaneCoord);
    // v1.y = 0.5 * (height - v1.y / maxPlaneCoord * width);

    return Vec3(
        1,
        -((xPixel * 2.0f / width - 1) * maxPlaneCoord),
        -((yPixel * 2.0f - height) / width * maxPlaneCoord)
    );
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

    Vec3 movement = Vec3(forwardMovement,sidewaysMovement,verticalMovement);
    movement.rotateZ(camera.getThetaZ());
    pos = pos + movement;
    camera.setPos(pos);

    camera.setThetaY(camera.getThetaY() + rotateY);
    camera.setThetaZ(camera.getThetaZ() + rotateZ);
}


void Game::renderRayTracing() {

    int width = pixelArray.getWidth();
    int height = pixelArray.getHeight();

    for (int y = 0; y < height; y++) {
        for (int x = 0; x < width; x++) {
            Vec3 dir = getPlaneCoords(x, y);
            dir.rotateY(camera.getThetaY());
            dir.rotateZ(camera.getThetaZ());
            
            Ray ray = Ray(camera.getPos(), dir);

            Vec3 center(5,0,0);
            float radius = 0.5f;

            Vec3 oc = center - ray.getOrigin();
            auto a = ray.getDirection().dot(ray.getDirection());
            auto b = -2.0 * ray.getDirection().dot(oc);
            auto c = oc.dot(oc) - radius*radius;
            auto discriminant = b*b - 4*a*c;
            bool hasIntersection = (discriminant >= 0);

            if (hasIntersection) {
                pixelArray.setPixel(x, y, 255, 255, 255);
            }
        }
    }
}