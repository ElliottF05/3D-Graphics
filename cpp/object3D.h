#pragma once

#include <vector>
#include "vec3.h"

class Object3D {
    private:
        std::vector<Vec3> vertices;
        int r, g, b;
        float reflectivity;
        bool isDeletable;
    public:
        Object3D();
        Object3D(std::vector<Vec3> vertices, int r, int g, int b, float reflectivity, bool isDeletable);

        const std::vector<Vec3>& getVertices() const;
        int getR() const;
        int getG() const;
        int getB() const;
};