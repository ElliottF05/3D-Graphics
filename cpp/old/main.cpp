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
static int selectedRed = 0;
static int selectedGreen = 0;
static int selectedBlue = 0;

// Setting up buffer
static uint8_t* buffer = new uint8_t[window.width * window.height * 4];

// Ghost object
static graphics::Object3D ghostObject;

extern "C" {
    EMSCRIPTEN_KEEPALIVE
    void EXTERN_setupScene() {
        graphics::Object3D floorGrid;
        floorGrid.isDeletable = false;
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
                floorGrid.triangles.push_back(t1);
                floorGrid.triangles.push_back(t2);
            }
        }
        graphics::Object3D::objects.push_back(floorGrid);

        graphics::Vec3 lightPos(-50, 0, 50);
        graphics::Light l1(lightPos, 0, -M_PI / 4.0, 10, 4000);
        graphics::Light::lights.push_back(l1);

        graphics::Object3D::objects.push_back(graphics::Object3D::buildCube(graphics::Vec3(0.5, -0.5, 0.5), 1));
        graphics::Object3D::objects.push_back(graphics::Object3D::buildSphere(graphics::Vec3(3.5, -0.5, 0.5), 1, 40, 255, 200, 200));

        for (graphics::Light &l : graphics::Light::lights) {
            for (graphics::Object3D &o : graphics::Object3D::objects) {
                l.fillZBuffer(o.triangles);
            }
        }

        cam.pos.y = -2;
        cam.pos.z = 2;
    }
}

extern "C" {
    EMSCRIPTEN_KEEPALIVE
    uint8_t* EXTERN_getBuffer() {
        auto start = std::chrono::high_resolution_clock::now();

        // CLEARING WINDOW
        window.clear();
        while (threads::threadPool.getNumberOfActiveTasks() > 0) {
            std::this_thread::sleep_for(std::chrono::microseconds(200));
        }

        // DRAWING TRIANGLES
        for (graphics::Object3D &o : graphics::Object3D::objects) {
            o.drawMultithreaded(cam, window);
        }
        while (threads::threadPool.getNumberOfActiveTasks() > 0) {
            std::this_thread::sleep_for(std::chrono::microseconds(200));
        }
        const graphics::Triangle* lookingAtTriangle = cam.lookingAtTriangle;
        const graphics::Object3D* lookingAtObject = cam.lookingAtObject;

        // DRAWING GHOST TRIANGLES
        ghostObject = graphics::Object3D::buildCube(cam.getPositionOfNewObject(window), 1, selectedRed, selectedGreen, selectedBlue);
        ghostObject.drawMultithreaded(cam, window);
        while (threads::threadPool.getNumberOfActiveTasks() > 0) {
            std::this_thread::sleep_for(std::chrono::microseconds(200));
        }

        cam.lookingAtTriangle = lookingAtTriangle;
        cam.lookingAtObject = lookingAtObject;

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
    void EXTERN_userInput(int cameraMoveFoward, int cameraMoveSide, int cameraMoveUp, int cameraRotateZ, int cameraRotateY, int userInputCode) {
        float moveMultiplier = 0.1;
        cam.moveRelative(moveMultiplier * cameraMoveFoward, moveMultiplier * cameraMoveSide, moveMultiplier * cameraMoveUp);
        float rotateMultiplier = 0.01;
        cam.rotate(rotateMultiplier * cameraRotateZ, rotateMultiplier * cameraRotateY);

        if (userInputCode == 1) {
            graphics::Object3D::objects.push_back(ghostObject);
            for (graphics::Light &l : graphics::Light::lights) {
                l.fillZBuffer(ghostObject.triangles);
            }
        } else if (userInputCode == 2) {
            graphics::Object3D::removeObject(*cam.lookingAtObject);
            for (graphics::Light &l : graphics::Light::lights) {
                l.zBuffer.clear();
                while (threads::threadPool.getNumberOfActiveTasks() > 0) {
                    std::this_thread::sleep_for(std::chrono::microseconds(200));
                }
                for (graphics::Object3D &o : graphics::Object3D::objects) {
                    l.fillZBuffer(o.triangles);
                }
            }
        }
    }
}

extern "C" {
    EMSCRIPTEN_KEEPALIVE
    void EXTERN_setSelectedColors(int r, int g, int b) {
        selectedRed = r;
        selectedGreen = g;
        selectedBlue = b;
    }
}


extern "C" {
    EMSCRIPTEN_KEEPALIVE
    int EXTERN_getDataBufferSize() {
        return graphics::utils::getDataBufferSize();
    }
}
extern "C" {
    EMSCRIPTEN_KEEPALIVE
    float* EXTERN_getDataBufferPointer() {
        return graphics::utils::getDataBufferPointer();
    }
}
extern "C" {
    EMSCRIPTEN_KEEPALIVE
    float* EXTERN_setDataBufferPointer(int size) {
        return graphics::utils::setDataBufferPointer(size);
    }
}
extern "C" {
    EMSCRIPTEN_KEEPALIVE
    void EXTERN_loadScene(float data[]) {
        graphics::utils::loadScene(data);
    }
}


int main(int, char**){
    std::cout << "Hello, from main!" << std::endl;
    return 0;
}