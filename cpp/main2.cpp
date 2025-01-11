#include <iostream>
#include <emscripten.h>
#include <stdint.h>

#include "game.h"

static Game g_Game;

int main() {
    std::cout << "Hello from main2.cpp!" << std::endl;
    g_Game.setupScene();
    g_Game.render();
}

void test() {
    g_Game.setupScene();
    g_Game.render();
}

// EXPORTED FUNCTION
extern "C" {
    EMSCRIPTEN_KEEPALIVE
    void renderScene() {
        g_Game.render();
    }

    EMSCRIPTEN_KEEPALIVE
    int renderSceneRayTracing(int startIndex) {
        return g_Game.renderRayTracing(startIndex);
    }

    EMSCRIPTEN_KEEPALIVE
    uint8_t* getImageBuffer() {
        // std::cout << "main2.cpp: getImageBuffer() called" << std::endl;
        return g_Game.exportImageBuffer();
    }

    EMSCRIPTEN_KEEPALIVE
    void userInput(float forwardMovement, float sidewaysMovement, float verticalMovement, float rotateZ, float rotateY, float otherInputCode) {
        // std::cout << "main2.cpp: userInput() called" << std::endl;
        // std::cout << "forwardMovement: " << forwardMovement << "sidewaysMovement: " << sidewaysMovement << "verticalMovement: " << verticalMovement << "rotateZ: " << rotateZ << "rotateY: " << rotateY << "otherInputCode: " << otherInputCode << std::endl;
        g_Game.userCameraInput(forwardMovement, sidewaysMovement, verticalMovement, rotateZ, rotateY, otherInputCode);
    }

    EMSCRIPTEN_KEEPALIVE
    float* getSceneDataBuffer() {
        return g_Game.getSceneDataBuffer();
    }

    EMSCRIPTEN_KEEPALIVE
    float* allocateSceneDataBuffer(int size) {
        return g_Game.allocateSceneDataBuffer(size);
    }

    EMSCRIPTEN_KEEPALIVE
    void loadSceneToCPP(float data[]) {
        g_Game.loadSceneToCPP(data);
    }

    EMSCRIPTEN_KEEPALIVE
    void setSelectedColors(int r, int g, int b) {
        g_Game.setSelectedColors(r,g,b);
    }
}