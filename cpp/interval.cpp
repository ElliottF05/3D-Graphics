#include "interval.h"
#include <algorithm>
#include <cmath>

// CONSTRUCTORS
Interval::Interval(float min, float max) : min(min), max(max) {}
Interval::Interval() : min(+INFINITY), max(-INFINITY) {} // default interval is empty

// METHODS
float Interval::size() const {
    return max - min;
}
bool Interval::contains(float value) const {
    return min <= value && value <= max;
}
bool Interval::surrounds(float value) const {
    return min < value && value < max;
}
float Interval::clamp(float value) const {
    return std::clamp(value, min, max);
}

// STATIC VARIABLES
const Interval Interval::empty = Interval(INFINITY, -INFINITY);
const Interval Interval::universe = Interval(-INFINITY, INFINITY);