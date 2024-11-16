#pragma once

#include <vector>
#include "light.h"
#include "object3D.h"
#include "camera.h"
#include "pixelArray.h"
#include "zBuffer.h"

class Game {
private:
    std::vector<Object3D> objects;
    std::vector<Light> lights;
    Camera camera;

    PixelArray pixelArray;
    ZBuffer zBuffer;

    uint8_t* imageBuffer;

    void fillTriangle(Vec3& v1, Vec3& v2, Vec3& v3, int r, int g, int b, float reflectivity, Vec3& normal);
    Vec3 getPlaneCoords(int xPixel, int yPixel);

public:
    Game();
    void setupScene();
    void render();
    void userCameraInput(float forwardMovement, float sidewaysMovement, float verticalMovement, float rotateZ, float rotateY, float otherInputCode);
    uint8_t* exportImageBuffer();
};