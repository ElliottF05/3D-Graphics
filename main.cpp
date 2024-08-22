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

        graphics::utils::buildCube(graphics::Vec3(0.5, -0.5, 0.5), 1, graphics::Triangle::triangles);

        for (graphics::Light &l : graphics::Light::lights) {
            l.fillZBuffer(graphics::Triangle::triangles);
        }

        cam.pos.y = -2;
        cam.pos.z = 1;
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
    return newObjectPosition;
}

extern "C" {
    EMSCRIPTEN_KEEPALIVE
    uint8_t* get_buffer() {
        auto start = std::chrono::high_resolution_clock::now();
        std::cout << cam.direction.toString() << std::endl;

        // CLEARING WINDOW
        window.clear();
        while (threads::threadPool.getNumberOfActiveTasks() > 0) {
            std::this_thread::sleep_for(std::chrono::microseconds(200));
        }

        // DRAWING TRIANGLES
        for (graphics::Triangle &t : graphics::Triangle::triangles) {
            threads::threadPool.addTask([&t] {
                t.draw(cam, window);
            });
        }
        while (threads::threadPool.getNumberOfActiveTasks() > 0) {
            std::this_thread::sleep_for(std::chrono::microseconds(200));
        }

        ghostTriangles.clear();
        graphics::utils::buildCube(getPositionOfNewObject(), 1, ghostTriangles, 120, 120, 120);
        for (graphics::Triangle &t : ghostTriangles) {
            threads::threadPool.addTask([&t] {
                t.draw(cam, window);
            });
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
        // std::cout << "returning buffer, elapsed time: " << elapsed.count() << std::endl;
        return &buffer[0];
    }
}

extern "C" {
    EMSCRIPTEN_KEEPALIVE
    void user_input(int cameraMoveFoward, int cameraMoveSide, int cameraMoveUp, int cameraRotateZ, int cameraRotateY, int userInputCode) {
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