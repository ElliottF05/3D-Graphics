#pragma once

struct Interval {
    float min, max;

    Interval(float min, float max);
    Interval(); // default interval will be empty

    float size() const;
    bool contains(float value) const;
    bool surrounds(float value) const;
    float clamp(float value) const;

    static const Interval empty;
    static const Interval universe;
};