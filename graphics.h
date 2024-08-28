#pragma once

#include <memory>
#include <vector>
#include <mutex>

namespace graphics {

//---------------------------------------------------------------------------
// FORWARD DECLARATION OF STRUCTS
struct Vec3;

struct Point;
struct Line;
struct Triangle;
struct Object3D;

struct Camera;
struct World;

struct PixelArray;
struct ZBuffer;
struct Window;

struct Light;


//---------------------------------------------------------------------------
// DECLARING "Vec3"
struct Vec3 {
    float x,y,z;

    // constructors
    Vec3(float x, float y, float z);
    Vec3();

    // operations
    Vec3 operator+(const Vec3& other) const;
    Vec3 operator-(const Vec3& other) const;
    Vec3 operator*(const float scalar) const;
    Vec3 operator/(const float scalar) const;
    Vec3& operator+=(const Vec3& vec);
    Vec3& operator-=(const Vec3& vec);
    Vec3& operator*=(const float scalar);
    Vec3& operator/=(const float scalar);
    Vec3 cross(const Vec3& other) const;
    float dot(const Vec3& other) const;
    float mag() const;
    void normalize();
    float angleWith(const Vec3& other) const;

    // specific functions
    void rotateZ(float thetaZ);
    void rotateY(float thetaY);
    void rotateZKnownTrig(float sinthetaZ, float costhetaZ);
    void rotateYKnownTrig(float sinthetaY, float costhetaY);
    void rotateKnownTrig(float sinthetaZ, float costhetaZ, float sinthetaY, float costhetaY);
    void rotate(float thetaZ, float thetaY);

    std::string toString();
};
// extra operators for Vec3
Vec3 operator*(const float scalar, const Vec3& vec);
Vec3 operator/(const float scalar, const Vec3& vec);



//---------------------------------------------------------------------------
// DECLARING "Point"
struct Point {
    Vec3 absolutePos, cameraPos, projectedPos, screenPos;
    float distToCamera;

    Point(Vec3 absolutePos);
    Point(float x, float y, float z);
    Point();

    void calculateCameraPos(const Camera& cam);
    void calculateProjectedPos();
    void calculateScreenPos(const Camera& cam, const Window& window);
    void calculateScreenPos(const Camera& cam, const int width, const int height);
    void calculateAll(const Camera& cam, const Window& window);

    void draw(const Camera& cam, const Window& window);

    std::string toString();
};


//---------------------------------------------------------------------------
// DECLARING "Line"
struct Line {
    Point p1, p2;

    Line(Point p1, Point p2);
    Line(Vec3 p1, Vec3 p2);
    Line();

    void draw(const Camera& cam, Window& window);
};


//---------------------------------------------------------------------------
// DECLARING "Triangle"
struct Triangle {
    static std::vector<Triangle> triangles;

    Point p1, p2, p3;
    Vec3 absoluteNormal;
    Vec3 cameraNormal;
    int r,g,b;

    Triangle(Vec3 p1, Vec3 p2, Vec3 p3, int r, int g, int b);
    Triangle(Point p1, Point p2, Point p3);
    Triangle(Vec3 p1, Vec3 p2, Vec3 p3);
    Triangle();

    void draw(Camera& cam, Window& window, const Object3D& object);

    static void drawVerticalScreenLine(Camera& cam, Window& window, const Triangle& triangle, const Object3D& object, int x, float y1, float y2, float d1);
    static void drawVerticalScreenLine(Camera& cam, Window& window, const std::shared_ptr<Triangle> triangle, const Object3D& object, int x, float y1, float y2, float d1);
};


//---------------------------------------------------------------------------
// DECLARING "Object3D"
struct Object3D {
    static std::vector<Object3D> objects;
    std::vector<Triangle> triangles;
    bool isDeletable;

    Object3D();
    Object3D(std::vector<Triangle> triangles);
    Object3D(std::vector<Triangle> triangles, bool isDeletable);

    void drawMultithreaded(Camera& cam, Window& window);

    // Making new objects
    static Object3D buildCube(Vec3 center, float sideLength, int r, int g, int b);
    static Object3D buildSphere(Vec3 center, float radius, int iterations, int r, int g, int b);
    static Object3D buildCube(Vec3 center, float sideLength);
    static Object3D buildSphere(Vec3 center, float radius, int iterations);
};



//---------------------------------------------------------------------------
// DECLARING "Camera"
struct Camera {
    Vec3 pos;
    float thetaZ, thetaY, sinthetaZ, sinthetaY, costhetaZ, costhetaY, fov, fov_rad, maxPlaneCoord, maxPlaneCoordInv;
    Vec3 direction;
    Vec3 floorDirection;
    const Triangle* lookingAtTriangle;
    const Object3D* lookigAtObject;

    Camera(Vec3 pos, float thetaZ, float thetaY, float fov);
    Camera();

    void moveRelative(float forward, float sideward, float upward);
    void rotate(float thetaZ, float thetaY);
    float getCameraYFromPixel(int x, int width) const;
    float getCameraZFromPixel(int y, int height) const;
    float getCameraYFromPixelFast(int x, float widthInv) const;
    float getCameraZFromPixelFast(int y, float heightInv) const;

    Vec3 getCenterOfViewPosition(Window& window) const;
    Vec3 getPositionOfNewObject(Window& window) const;
};


//---------------------------------------------------------------------------
// DECLARING "World"
struct World { // TODO: incomplete and unused
    Camera cam;
    Vec3 sunDirection;
};



//---------------------------------------------------------------------------
// DECLARING "PixelArray"
struct PixelArrayData {
    int r, g ,b;
    std::mutex mutex;

    PixelArrayData(int r, int g, int b);
    PixelArrayData(int color);
    PixelArrayData(const PixelArrayData& other);
    PixelArrayData();
};
struct PixelArray {
    int width, height;
    std::vector<PixelArrayData> data;

    PixelArray(int width, int height);

    int getIndex(int x, int y);
    void setPixel(int x, int y, int color);
    void setPixel(int x, int y, int r, int g, int b);
    void clear();
};


//---------------------------------------------------------------------------
// DECLARING "ZBuffer"
struct ZBufferData {
    float depth;
    std::mutex mutex;

    ZBufferData(float depth);
    ZBufferData(const ZBufferData& other);
    ZBufferData();
};
struct ZBuffer {
    int width, height;
    std::vector<ZBufferData> data;

    ZBuffer(int width, int height);

    int getIndex(int x, int y);
    void setDepth(int x, int y, float depth);
    float getDepth(int x, int y);
    void clear();
};


//---------------------------------------------------------------------------
// DECLARING "Window"
struct Window {
    int width, height;
    float widthInv, heightInv;
    PixelArray pixelArray;
    ZBuffer zBuffer;

    Window(int width, int height);

    void drawPoint(Point& point);
    void drawLine(Line& line);
    void drawTriangle(Triangle& triangle, const Object3D& object, Camera& cam);
    void drawTriangle(std::shared_ptr<Triangle> triangle, const Object3D& object, Camera& cam);
    void draw(); // implementation specific
    void getUint8Pointer(uint8_t* buffer); // implementation specific
    void clear();
};


//---------------------------------------------------------------------------
// DECLARING "Light"
struct Light {
    Camera cam;
    ZBuffer zBuffer;
    int filteringRadius;
    float luminosity, filteringAreaInv;

    static std::vector<Light> lights;

    Light(Vec3 pos, float thetaZ, float thetaY, float fov, float luminosity);
    Light(Vec3 pos, float thetaZ, float thetaY, float luminosity);

    void getTrianglePerspectiveFromLight(Triangle triangle);
    void addTriangleToZBuffer(Triangle& triangle);
    void fillZBuffer(std::vector<Triangle>& triangles);

    float amountLit(Vec3& vec, float& vecToLightMagInv);

};

//---------------------------------------------------------------------------
// DECLARING "utils"
namespace utils {
    // using floats
    void sortPair(float& toLower, float& toHigher);
    void clampToRange(float& value, float min, float max);
    void clampToRange(float& value, float max);
    void sortAndClamp(float& toLower, float& toHigher, float min, float max);
    void sortAndClamp(float& toLower, float& toHigher, float max);

    // using ints
    void sortPair(int& toLower, int& toHigher);
    void clampToRange(int& value, int max);
    void sortAndClamp(int& toLower, int& toHigher, int max);
}

}