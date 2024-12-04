#pragma once

#include <vector>
#include "light.h"
#include "object3D.h"
#include "camera.h"
#include "pixelArray.h"
#include "zBuffer.h"
#include "ray.h"
#include "sphere.h"

class Game {
private:
    std::vector<Object3D> objects;
    std::vector<Light> lights;
    Camera camera;

    PixelArray pixelArray;
    ZBuffer zBuffer;

    uint8_t* imageBuffer;

    void projectTriangleBatch(std::vector<Vec3>& vertices, std::vector<const ObjectProperties*>& properties, int start, int end);
    void projectTriangle(Vec3& v1, Vec3& v2, Vec3& v3, const ObjectProperties& properties);

    void fillTriangle(Vec3& v1, Vec3& v2, Vec3& v3, const ObjectProperties& properties, Vec3& normal);
    void fillTriangleOwned(Vec3 v1, Vec3 v2, Vec3 v3, const ObjectProperties& properties, Vec3 normal);

    void fillHorizontalLine(int y, float x1, float x2, float invLeftDepth, float invRightDepth, const ObjectProperties& properties, Vec3 normal);

    Vec3 getPlaneCoords(int xPixel, int yPixel);
    Vec3 getPlaneCoords(float xPixel, float yPixel);

    Ray spawnRay(float xPixel, float yPixel);

public:
    Game();
    void setupScene();
    void render();
    void renderRayTracing();
    void userCameraInput(float forwardMovement, float sidewaysMovement, float verticalMovement, float rotateZ, float rotateY, float otherInputCode);
    uint8_t* exportImageBuffer();
};