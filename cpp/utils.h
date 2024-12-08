#pragma once

#include <random>
#include "vec3.h"

struct Vec3;

float randomFloat();
float randomFloat(float min, float max);
Vec3 randomInUnitDisk();

float degreesToRadians(float degrees);