#pragma once

#include <vector>
#include "object3D.h"
#include "camera.h"
#include "pixelArray.h"

class Game {
private:
    std::vector<Object3D> objects;
    PixelArray pixelArray;
    Camera camera;

    uint8_t* imageBuffer;

    void fillTriangle(Vec3& v1, Vec3& v2, Vec3& v3, int r, int g, int b);

public:
    Game();
    void setupScene();
    void render();
    void userCameraInput(float forwardMovement, float sidewaysMovement, float verticalMovement, float rotateZ, float rotateY, float otherInputCode);
    uint8_t* exportImageBuffer();
};