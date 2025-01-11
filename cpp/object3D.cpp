#include "object3D.h"

// ObjectProperties
ObjectProperties::ObjectProperties(int r, int g, int b, float k_s, float k_d, float k_a, int shininess, bool isDeletable) : r(r), g(g), b(b), k_s(k_s), k_d(k_d), k_a(k_a), shininess(shininess), isDeletable(isDeletable) {}
ObjectProperties::ObjectProperties() : r(255), g(255), b(255), k_s(0), k_d(1), k_a(0.2f), shininess(0), isDeletable(true) {}

// CONSTRUCTORS
int Object3D::numObjects = 0;

Object3D::Object3D() : vertices(), properties(255, 255, 255, 0, 1, 0.2f, 0, true) {
    id = numObjects;
    numObjects++;
}
Object3D::Object3D(std::vector<Vec3> vertices, ObjectProperties properties) : vertices(vertices), properties(properties) {
    id = numObjects;
    numObjects++;
}
Object3D::Object3D(std::vector<Vec3> vertices, int r, int g, int b) : vertices(vertices), properties(r, g, b, 0, 1, 0.2f, 0, true) {
    id = numObjects;
    numObjects++;
}
Object3D::Object3D(std::vector<Vec3> vertices, int r, int g, int b, float k_s, float k_d, float k_a, int shininess, bool isDeletable) : vertices(vertices), properties(r, g, b, k_s, k_d, k_a, shininess, isDeletable) {
    id = numObjects;
    numObjects++;
}

const std::vector<Vec3>& Object3D::getVertices() const {
    return vertices;
}
std::vector<Vec3>& Object3D::getMutableVertices() {
    return vertices;
}
const ObjectProperties& Object3D::getProperties() const {
    return properties;
}
ObjectProperties& Object3D::getMutableProperties() {
    return properties;
}
int Object3D::getId() const {
    return id;
}

