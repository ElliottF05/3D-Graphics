#include "game.h"
#include "camera.h"
#include "hitRecord.h"
#include "interval.h"
#include "object3D.h"
#include "ray.h"
#include "sphere.h"
#include "threads.h"
#include "utils.h"
#include "vec3.h"
#include "zBuffer.h"
#include <algorithm>
#include <chrono>
#include <cmath>
#include <functional>
#include <iostream>
#include <sys/ttydefaults.h>
#include <vector>

// CONSTRUCTOR
Game::Game() : pixelArray(WINDOW_WIDTH, WINDOW_HEIGHT), zBuffer(WINDOW_WIDTH, WINDOW_HEIGHT), camera(Vec3(0,0,0), 0, 0, CAMERA_FOV), rtCamera(Vec3(0,0,0), 0, 0, CAMERA_FOV) {
    imageBuffer = new uint8_t[WINDOW_WIDTH * WINDOW_HEIGHT * 4];
}

void Game::setupScene() {
    std::cout << "game.cpp: setupScene() called" << std::endl;

    // make grid
    std::vector<Vec3> darkGrey;
    std::vector<Vec3> lightGrey;
    int gridRadius = 5;
    for (int i = -gridRadius; i < gridRadius; i++) {
        for (int j = -gridRadius; j < gridRadius; j++) {
            std::vector<Vec3>& addingTo = ((i + j) % 2 == 0) ? lightGrey : darkGrey;
            addingTo.emplace_back(i,j,0);
            addingTo.emplace_back(i+1,j+1,0);
            addingTo.emplace_back(i+1,j,0);
            addingTo.emplace_back(i,j,0);
            addingTo.emplace_back(i,j+1,0);
            addingTo.emplace_back(i+1,j+1,0);
        }
    }
    objects.emplace_back(darkGrey, 140, 140, 140, 1.0f, 1.0f, 0.2f, 20, true);
    objects.emplace_back(lightGrey, 200, 200, 200, 1.0f, 1.0f, 0.2f, 20, true);

    std::vector<Vec3> testObjVertices;
    float radius = 0.5f;
    int iterations = 100;

    Vec3 center = Vec3(1,1,radius);

    std::vector<Vec3> prev;
    for (int i = 0; i < iterations; i++) {
        prev.push_back(center + Vec3(0,0,radius));
    }
    std::vector<Vec3> curr;
    for (int i = 1; i < iterations; i++) {
        float z = radius * std::sin(M_PI / 2.0f - i * M_PI / iterations);
        for (int j = 0; j < iterations; j++) {
            float x = radius * std::cos(j * 2.0f * M_PI / iterations) * std::cos(M_PI / 2.0f - i * M_PI / iterations);
            float y = radius * std::sin(j * 2.0f * M_PI / iterations) * std::cos(M_PI / 2.0f - i * M_PI / iterations);
            curr.push_back(center + Vec3(x,y,z));
        }
        for (int i = 0; i < iterations; i++) {
            testObjVertices.push_back(prev[i]);
            testObjVertices.push_back(curr[(i+1) % iterations]);
            testObjVertices.push_back(curr[i]);
            testObjVertices.push_back(prev[i]);
            testObjVertices.push_back(prev[(i+1) % iterations]);
            testObjVertices.push_back(curr[(i+1) % iterations]);
        }
        prev = curr;
        curr.clear();
    }

    Object3D testObj = Object3D(testObjVertices, 220, 220, 220, 1.0f, 1.0f, 0.2f, 20, true);
    objects.push_back(testObj);

    // create light
    lights.emplace_back(Vec3(-10,0,10), 0, -M_PI/4.0f, M_PI/4.0f, 255, 255, 255, 12);
    lights[0].resetShadowMap();
    lights[0].addObjectsToShadowMap(objects);

    // create camera
    camera = Camera(Vec3(0.0000111f,0.0000111f,1.0000111), 0.0000111f, 0.0000111f, M_PI/2.0f);


    // setting up ray tracing
    auto ground_material = std::make_shared<Lambertian>(Vec3(0.5, 0.5, 0.5));
    spheres.emplace_back(Vec3(0,0,-1000), 1000, ground_material);

    for (int a = -11; a < 11; a++) {
        for (int b = -11; b < 11; b++) {
            auto choose_mat = randomFloat();
            Vec3 center(a + 0.9*randomFloat(), b + 0.9*randomFloat(), 0.2);

            if ((center - Vec3(4, 0, 0.2)).length() > 0.9) {
                std::shared_ptr<Material> sphere_material;

                if (choose_mat < 0.8) {
                    // diffuse
                    auto albedo = Vec3(randomFloat(), randomFloat(), randomFloat()) * Vec3(randomFloat(), randomFloat(), randomFloat());
                    sphere_material = std::make_shared<Lambertian>(albedo);
                    spheres.emplace_back(center, 0.2, sphere_material);
                } else if (choose_mat < 0.95) {
                    // metal
                    auto albedo = Vec3(0.5 * (1 + randomFloat()), 0.5 * (1 + randomFloat()), 0.5 * (1 + randomFloat()));
                    auto fuzz = randomFloat(0, 0.5);
                    sphere_material = std::make_shared<Metal>(albedo, fuzz);
                    spheres.emplace_back(center, 0.2, sphere_material);
                } else {
                    // glass
                    sphere_material = std::make_shared<Dielectric>(1.5);
                    spheres.emplace_back(center, 0.2, sphere_material);
                }
            }
        }
    }

    auto material1 = std::make_shared<Dielectric>(1.5);
    spheres.emplace_back(Vec3(0,0,1), 1, material1);

    auto material2 = std::make_shared<Lambertian>(Vec3(0.4, 0.2, 0.1));
    spheres.emplace_back(Vec3(-4, 0, 1), 1, material2);

    auto material3 = std::make_shared<Metal>(Vec3(0.7, 0.6, 0.5), 0.0);
    spheres.emplace_back(Vec3(4, 0, 1), 1, material3);

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
    // std::cout << "total frame time: " << totalDuration.count() << std::endl;
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

                Vec3 worldPos = getPlaneCoords(x, y, camera) * depth;
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

                Vec3 worldPos = getPlaneCoords(x, y, camera) * depth;
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

            Vec3 worldPos = getPlaneCoords(x, y, camera) * depth;
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

Vec3 Game::getPlaneCoords(int xPixel, int yPixel, Camera& cam) {
    int width = pixelArray.getWidth();
    int height = pixelArray.getHeight();
    float maxPlaneCoord = tan(cam.getFov() / 2.0f);

    // v1.x = (0.5 * width) * (1 - v1.x / maxPlaneCoord);
    // v1.y = 0.5 * (height - v1.y / maxPlaneCoord * width);

    return Vec3(
        1,
        -((xPixel * 2.0f / width - 1) * maxPlaneCoord),
        -((yPixel * 2.0f - height) / width * maxPlaneCoord)
    );
}
Vec3 Game::getPlaneCoords(float xPixel, float yPixel, Camera &cam) {
    int width = pixelArray.getWidth();
    int height = pixelArray.getHeight();
    float maxPlaneCoord = tan(cam.getFov() / 2.0f);

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

float* Game::getSceneDataBuffer() {
    // for now, just keep track of camera pos, camera angle, objects3D's

    // first element gives final length of array, reserving space for this
    sceneDataBuffer.push_back(0.0f); 

    // camera pos
    sceneDataBuffer.push_back(camera.getPos().x);
    sceneDataBuffer.push_back(camera.getPos().y);
    sceneDataBuffer.push_back(camera.getPos().z);

    // camera angles
    sceneDataBuffer.push_back(camera.getThetaZ());
    sceneDataBuffer.push_back(camera.getThetaY());

    // number of object3D's
    sceneDataBuffer.push_back(objects.size());

    for (auto& obj : objects) {
        sceneDataBuffer.push_back(obj.getProperties().r);
        sceneDataBuffer.push_back(obj.getProperties().g);
        sceneDataBuffer.push_back(obj.getProperties().b);

        sceneDataBuffer.push_back(obj.getProperties().k_s);
        sceneDataBuffer.push_back(obj.getProperties().k_d);
        sceneDataBuffer.push_back(obj.getProperties().k_a);

        sceneDataBuffer.push_back(obj.getProperties().shininess);

        sceneDataBuffer.push_back(obj.getProperties().isDeletable);

        sceneDataBuffer.push_back(obj.getVertices().size());
        for (auto& v : obj.getVertices()) {
            sceneDataBuffer.push_back(v.x);
            sceneDataBuffer.push_back(v.y);
            sceneDataBuffer.push_back(v.z);
        }
    }

    // update first element, which holds size of array
    sceneDataBuffer[0] = sceneDataBuffer.size();
    
    return &sceneDataBuffer[0];
}

float* Game::allocateSceneDataBuffer(int size) {
    sceneDataBuffer.clear();
    sceneDataBuffer.resize(size*2);

    return &sceneDataBuffer[0];
}
void Game::loadSceneToCPP(float data[]) {

    setupScene();
    objects.clear();

    int size = data[0];
    int i = 1;

    camera.setPos(Vec3(data[i], data[i+1], data[i+2]));
    i += 3;

    camera.setThetaZ(data[i]);
    camera.setThetaY(data[i+1]);
    i += 2;

    int numObjects = data[i];
    i += 1;

    for (int j = 0; j < numObjects; j++) {
        ObjectProperties properties = ObjectProperties(data[i], data[i+1], data[i+2], data[i+3], data[i+4], data[i+5], data[i+6], data[i+7]);
        i += 8;

        int numVertices = data[i];
        i += 1;

        std::vector<Vec3> vertices;
        for (int k = 0; k < numVertices; k++) {
            vertices.push_back(Vec3(data[i], data[i+1], data[i+2]));
            i += 3;
        }

        objects.push_back(Object3D(vertices, properties));
    }
}




int Game::renderRayTracing(int startIndex) {
    // Made by mostly following guide at https://raytracing.github.io/books/RayTracingInOneWeekend.html

    auto startTime = std::chrono::high_resolution_clock::now();

    int width = pixelArray.getWidth();
    int height = pixelArray.getHeight();

    rtCamera.setFovDegrees(20);
    rtCamera.setPos(Vec3(13,3,2));
    rtCamera.lookAt(Vec3(0,0,0));

    rtCamera.setDefocusAngleDegrees(0.6);
    rtCamera.setFocusDistance(10.0);

    int startY = startIndex / width;
    int startX = startIndex % width;
    std::cout << startY << ", " << startX << std::endl;

    for (int y = startY; y < height; y++) {
        if (y != startY) {
            startX = 0;
        }
        for (int x = startX; x < width; x++) {
            auto currTime = std::chrono::high_resolution_clock::now();
            auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(currTime - startTime);
            if (elapsed.count() > 500) {
                threadPool.waitUntilTasksFinished();
                return y * width + x;
            }

            threadPool.addTask([this, x, y] {
                rayTracePixel(x, y);
            });

            // Vec3 pixelColor(0,0,0);
            // for (int sample = 0; sample < RAY_SAMPLES_PER_PIXEL; sample++) {
            //     Ray ray = spawnRayAtPixel(x, y);
            //     pixelColor += traceRay(ray, MAX_RAY_DEPTH, spheres); // COLOR IS IN [0,1] RANGE
            // }

            // if (pixelColor.x == 0 && pixelColor.y == 0 && pixelColor.z == 0) {
            //     continue;
            // }

            // pixelColor /= RAY_SAMPLES_PER_PIXEL;
            // gammaCorrect(pixelColor);
            // transformColorTo255Range(pixelColor);

            // pixelArray.setPixel(x, y, pixelColor.x, pixelColor.y, pixelColor.z);
        }
    }
    return -1;
}

void Game::rayTracePixel(int xPixel, int yPixel) {
    Vec3 pixelColor(0,0,0);
    for (int sample = 0; sample < RAY_SAMPLES_PER_PIXEL; sample++) {
        Ray ray = spawnRayAtPixel(xPixel, yPixel);
        pixelColor += traceRay(ray, MAX_RAY_DEPTH, spheres); // COLOR IS IN [0,1] RANGE
    }

    if (pixelColor.x == 0 && pixelColor.y == 0 && pixelColor.z == 0) {
        return;
    }

    pixelColor /= RAY_SAMPLES_PER_PIXEL;
    gammaCorrect(pixelColor);
    transformColorTo255Range(pixelColor);

    pixelArray.setPixel(xPixel, yPixel, pixelColor.x, pixelColor.y, pixelColor.z);
}

Ray Game::spawnRayAtPixel(float xPixel, float yPixel) {

    xPixel += -0.5 + randomFloat();
    yPixel += -0.5 + randomFloat();

    Vec3 pixelPos = rtCamera.getFocusDistance() * getPlaneCoords(xPixel, yPixel, rtCamera);
    pixelPos.rotateY(rtCamera.getThetaY());
    pixelPos.rotateZ(rtCamera.getThetaZ());
    pixelPos += rtCamera.getPos();

    Vec3 rayOrigin = rtCamera.getPos();

    if (rtCamera.getDefocusAngle() != 0) {
        float defocusRadius = rtCamera.getFocusDistance() * std::tan(rtCamera.getDefocusAngle() / 2.0f);
        Vec3 defocusOffset = randomInUnitDisk() * defocusRadius;
        defocusOffset.z = defocusOffset.x;
        defocusOffset.x = 0;
        defocusOffset.rotateY(rtCamera.getThetaY());
        defocusOffset.rotateZ(rtCamera.getThetaZ());
        rayOrigin += defocusOffset;
    }

    Vec3 rayDirection = pixelPos - rayOrigin;
    return Ray(rayOrigin, rayDirection);
}

Vec3 Game::traceRay(const Ray& ray, int depth, const std::vector<Sphere>& spheres) {
    // COLORS ARE IN [0,1] RANGE
    if (depth < 0) {
        return Vec3(0,0,0);
    }

    HitRecord hitRecord;
    bool hitAnything = false;
    Interval hitInterval(0.001f, INFINITY);

    for (const Sphere& sphere : spheres) {
        if (sphere.rayHit(ray, hitInterval, hitRecord)) {
            hitAnything = true;
            hitInterval.max = hitRecord.t;
        }
    }

    if (hitAnything) {
        Ray rayOut;
        Vec3 attenuation;
        if (hitRecord.material->scatter(ray, hitRecord, attenuation, rayOut)) {
            return attenuation * traceRay(rayOut, depth - 1, spheres);
        } else {
            return Vec3(0,0,0);
        }
    }

    // Return the sky color in the case of no hit, COLORS ARE IN [0,1] RANGE
    Vec3 direction = ray.getDirection().normalized();
    float a = 0.5 * (direction.y + 1.0);
    return (1.0 - a) * Vec3(1.0, 1.0, 1.0) + a * Vec3(0.5, 0.7, 1.0); 
}

void Game::gammaCorrect(Vec3& color) {
    color.x = std::sqrt(color.x);
    color.y = std::sqrt(color.y);
    color.z = std::sqrt(color.z);
}
void Game::transformColorTo255Range(Vec3& color) {
    color *= 255;
    color.x = std::floor(color.x);
    color.y = std::floor(color.y);
    color.z = std::floor(color.z);
}