#pragma once

#include <vector>
#include "light.h"
#include "object3D.h"
#include "camera.h"
#include "pixelArray.h"
#include "zBuffer.h"
#include "ray.h"
#include "sphere.h"
#include "utils.h"

class Game {
private:
    std::vector<Object3D> objects;
    std::vector<Light> lights;
    std::vector<Sphere> spheres;
    Camera camera;
    Camera rtCamera;

    PixelArray pixelArray;
    ZBuffer zBuffer;

    uint8_t* imageBuffer;

    std::vector<float> sceneDataBuffer;

    void projectTriangleBatch(std::vector<Vec3>& vertices, std::vector<const ObjectProperties*>& properties, int start, int end);
    void projectTriangle(Vec3& v1, Vec3& v2, Vec3& v3, const ObjectProperties& properties);

    void fillTriangle(Vec3& v1, Vec3& v2, Vec3& v3, const ObjectProperties& properties, Vec3& normal);
    void fillTriangleOwned(Vec3 v1, Vec3 v2, Vec3 v3, const ObjectProperties& properties, Vec3 normal);

    void fillHorizontalLine(int y, float x1, float x2, float invLeftDepth, float invRightDepth, const ObjectProperties& properties, Vec3 normal);

    Vec3 getPlaneCoords(int xPixel, int yPixel, Camera& cam);
    Vec3 getPlaneCoords(float xPixel, float yPixel, Camera& cam);

    void rayTracePixel(int xPixel, int yPixel);

    Ray spawnRayAtPixel(float xPixel, float yPixel);
    Vec3 traceRay(const Ray& ray, int depth, const std::vector<Sphere>& spheres); // USES COLORS IN [0,1] RANGE

    void gammaCorrect(Vec3& color);
    void transformColorTo255Range(Vec3& color);


public:

    // CONSTANTS
    static const int WINDOW_WIDTH = 500;
    static const int WINDOW_HEIGHT = 500;
    static const int CAMERA_FOV = 90 * (M_PI / 180.0f);
    static const int RAY_SAMPLES_PER_PIXEL = 200;
    static const int MAX_RAY_DEPTH = 50;

    Game();
    void setupScene();
    void render();
    int renderRayTracing(int startIndex);
    void userCameraInput(float forwardMovement, float sidewaysMovement, float verticalMovement, float rotateZ, float rotateY, float otherInputCode);
    uint8_t* exportImageBuffer();


    // exporting from cpp
    float* getSceneDataBuffer();

    // importing to cpp
    float* allocateSceneDataBuffer(int size);
    void loadSceneToCPP(float data[]);

    
};