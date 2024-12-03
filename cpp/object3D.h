#pragma once

#include <vector>
#include "vec3.h"

struct ObjectProperties {
    int r, g, b;
    float k_s, k_d, k_a;
    int shininess;
    bool isDeletable;

    ObjectProperties(int r, int g, int b, float k_s, float k_d, float k_a, int shininess, bool isDeletable);
    ObjectProperties();
};

class Object3D {
    private:
        std::vector<Vec3> vertices;
        ObjectProperties properties;
    public:
        Object3D();
        Object3D(std::vector<Vec3> vertices, ObjectProperties properties);
        Object3D(std::vector<Vec3> vertices, int r, int g, int b, float k_s, float k_d, float k_a, int shininess, bool isDeletable);
        Object3D(std::vector<Vec3> vertices, int r, int g, int b);

        const std::vector<Vec3>& getVertices() const;
        std::vector<Vec3>& getMutableVertices();
        const ObjectProperties& getProperties() const;
        ObjectProperties& getMutableProperties();
};