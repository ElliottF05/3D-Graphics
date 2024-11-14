#include "object3D.h"

// CONSTRUCTORS
Object3D::Object3D() : vertices(), r(0), g(0), b(0), reflectivity(0), isDeletable(true) {}
Object3D::Object3D(std::vector<Vec3> vertices, int r, int g, int b, float reflectivity, bool isDeletable) : vertices(vertices), r(r), g(g), b(b), reflectivity(reflectivity), isDeletable(isDeletable) {}

const std::vector<Vec3>& Object3D::getVertices() const {
    return vertices;
}
int Object3D::getR() const {
    return r;
}
int Object3D::getG() const {
    return g;
}
int Object3D::getB() const {
    return b;
}

