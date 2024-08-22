#include <__chrono/duration.h>
#include <cmath>
#include <cstdlib>
#include <iostream>
#include <chrono>
#include <pthread.h>
#include <thread>
#include <vector>
#include "graphics.h"
#include "threads.h"
#include <emscripten.h>


// Setting up simulation necessities
static bool running = true;
static graphics::Window window(500, 500);
static graphics::Camera cam;

// Setting up buffer
static uint8_t* buffer = new uint8_t[window.width * window.height * 4];

// Setting up ghost triangles
static std::vector<graphics::Triangle> ghostTriangles;

extern "C" {
    EMSCRIPTEN_KEEPALIVE
    void setup_scene() {
        graphics::Point p1(-5, 10, -1), p2(-5, -10, -1), p3(15, 0, -1), p4(1, -5, -1);
        graphics::Triangle t1(p1, p3, p2);
        t1.g = 255;
        t1.b = 230;
        graphics::Triangle::triangles.push_back(t1);

        graphics::Vec3 not_used(-10, 0, 10);
        graphics::Light l1(not_used, 0, -M_PI / 4.0, 200);
        graphics::Light::lights.push_back(l1);

        for (graphics::Light &l : graphics::Light::lights) {
            l.fillZBuffer(graphics::Triangle::triangles);
        }

        // Spawn sphere logic
        int iterations = 20;
        std::vector<graphics::Vec3> prev(iterations);
        std::vector<graphics::Vec3> curr(iterations);
        bool onFirst = true;
        for (float thetaY = -M_PI / 2.0; thetaY <= M_PI / 2.0; thetaY += M_PI / iterations) {
            prev = curr;
            curr.clear();
            for (float thetaZ = 0; thetaZ <= 2 * M_PI; thetaZ += 2 * M_PI / iterations) {
                graphics::Vec3 v(std::cos(thetaY) * std::cos(thetaZ) + 2 * cam.floorDirection.x, std::cos(thetaY) * std::sin(thetaZ) + 2 * cam.floorDirection.y, std::sin(thetaY));
                v += cam.pos;
                curr.push_back(v);
            }
            if (onFirst) {
                onFirst = false;
                continue;
            }
            for (int i = 0; i < prev.size(); i++) {
                graphics::Triangle t1(prev[i], curr[i], curr[(i + 1) % iterations]);
                graphics::Triangle t2(prev[(i + 1) % iterations], prev[i], curr[(i + 1) % iterations]);
                t1.r = 255;
                t1.g = 255;
                t1.b = 255;
                t2.r = 255;
                t2.g = 255;
                t2.b = 255;
                graphics::Triangle::triangles.push_back(t1);
                graphics::Triangle::triangles.push_back(t2);
            }
        }

        for (graphics::Light &l : graphics::Light::lights) {
            l.fillZBuffer(graphics::Triangle::triangles);
        }

        cam.pos.y = -2;
        cam.pos.z = 1;
        cam.rotate(M_PI / 6.0, -M_PI / 8.0);
    }
}

graphics::Vec3 getCenterOfScreen() {
    return cam.pos + cam.direction * window.zBuffer.getDepth(window.width / 2, window.height / 2);
}

graphics::Vec3 getPositionOfNewObject() {
    graphics::Vec3 newObjectPosition = getCenterOfScreen() - cam.direction;
    newObjectPosition.x = round(newObjectPosition.x + 0.5) - 0.5;
    newObjectPosition.y = round(newObjectPosition.y + 0.5) - 0.5;
    newObjectPosition.z = round(newObjectPosition.z + 0.5) - 0.5;
}

extern "C" {
    EMSCRIPTEN_KEEPALIVE
    uint8_t* get_buffer() {
        auto start = std::chrono::high_resolution_clock::now();
        // graphics::Vec3 = getPositionOfNewObject();
        window.clear();
        while (threads::threadPool.getNumberOfActiveTasks() > 0) {
            std::this_thread::sleep_for(std::chrono::microseconds(200));
        }
        for (graphics::Triangle &t : graphics::Triangle::triangles) {
            threads::threadPool.addTask([&t] {
                t.draw(cam, window);
            });
            // t.draw(cam, window);
        }
        while (threads::threadPool.getNumberOfActiveTasks() > 0) {
            std::this_thread::sleep_for(std::chrono::microseconds(200));
        }
        auto end = std::chrono::high_resolution_clock::now();
        std::chrono::duration<double> elapsed = end - start;
        window.getUint8Pointer(buffer);
        while (threads::threadPool.getNumberOfActiveTasks() > 0) {
            std::this_thread::sleep_for(std::chrono::microseconds(200));
        }
        std::cout << "returning buffer, elapsed time: " << elapsed.count() << std::endl;
        return &buffer[0];
    }
}

extern "C" {
    EMSCRIPTEN_KEEPALIVE
    void user_input(int cameraMoveFoward, int cameraMoveSide, int cameraMoveUp, int cameraRotateZ, int cameraRotateY) {
        float moveMultiplier = 0.1;
        cam.moveRelative(moveMultiplier * cameraMoveFoward, moveMultiplier * cameraMoveSide, moveMultiplier * cameraMoveUp);
        float rotateMultiplier = 0.01;
        cam.rotate(rotateMultiplier * cameraRotateZ, rotateMultiplier * cameraRotateY);
    }
}

int main(int, char**){
    std::cout << "Hello, from main!" << std::endl;
    return 0;
}