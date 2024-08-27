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
        bool floorGridColor = true;
        int floorGridSize = 12;
        for (int i = -floorGridSize / 2; i < floorGridSize / 2; i++) {
            floorGridColor = !floorGridColor;
            for (int j = -floorGridSize / 2; j < floorGridSize / 2; j++) {
                floorGridColor = !floorGridColor;
                graphics::Point p1(i, j, 0);
                graphics::Point p2(i+1, j, 0);
                graphics::Point p3(i, j+1, 0);
                graphics::Point p4(i+1, j+1, 0);

                graphics::Triangle t1(p4, p2, p1);
                graphics::Triangle t2(p1, p3, p4);
                if (floorGridColor) {
                    t1.r = 200;
                    t1.g = 200;
                    t1.b = 200;
                    t2.r = 200;
                    t2.g = 200;
                    t2.b = 200;
                } else {
                    t1.r = 150;
                    t1.g = 150;
                    t1.b = 150;
                    t2.r = 150;
                    t2.g = 150;
                    t2.b = 150;
                }
                graphics::Triangle::triangles.push_back(t1);
                graphics::Triangle::triangles.push_back(t2);
            }
        }
        graphics::Vec3 lightPos(-50, 0, 50);
        graphics::Light l1(lightPos, 0, -M_PI / 4.0, 10, 4000);
        graphics::Light::lights.push_back(l1);

        graphics::utils::buildCube(graphics::Vec3(0.5, -0.5, 0.5), 1, graphics::Triangle::triangles);
        graphics::utils::buildSphere(graphics::Vec3(3.5, -0.5, 0.5), 1, 40, graphics::Triangle::triangles, 255, 200, 200);

        for (graphics::Light &l : graphics::Light::lights) {
            l.fillZBuffer(graphics::Triangle::triangles);
        }

        cam.pos.y = -2;
        cam.pos.z = 2;
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
        std::cout << "returning buffer, elapsed time: " << elapsed.count() << std::endl;
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

        if (userInputCode == 1) {
            for (graphics::Triangle &t : ghostTriangles) {
                graphics::Triangle::triangles.push_back(t);
            }
            for (graphics::Light &l : graphics::Light::lights) {
            l.fillZBuffer(ghostTriangles);
            }
        }
    }
}

int main(int, char**){
    std::cout << "Hello, from main!" << std::endl;
    return 0;
}