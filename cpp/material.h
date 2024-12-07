#pragma once

#include "ray.h"
#include "hitRecord.h"

struct HitRecord;

class Material {
    public: 
        virtual ~Material();
        virtual bool scatter(const Ray& rayIn, const HitRecord& hitRecord, Vec3& attenuation, Ray& rayOut) const;
};

class Lambertian : public Material {
    private:
        Vec3 albedo;
    public:
        Lambertian(const Vec3& albedo);
        bool scatter(const Ray& rayIn, const HitRecord& hitRecord, Vec3& attenuation, Ray& rayOut) const override;
};

class Metal : public Material {
    private:
        Vec3 albedo;
        float fuzziness;
    public:
        Metal(const Vec3& albedo, float fuzziness);
        Metal(const Vec3& albedo);
        bool scatter(const Ray& rayIn, const HitRecord& hitRecord, Vec3& attenuation, Ray& rayOut) const override;
};

class Dielectric : public Material {
    private:
        Vec3 color;
        float refractionIndex;

        static float reflectance(float cosTheta, float n1, float n2);
    public:
        Dielectric(float refractionIndex);
        Dielectric(const Vec3& color, float refractionIndex);
        bool scatter(const Ray& rayIn, const HitRecord& hitRecord, Vec3& attenuation, Ray& rayOut) const override;
};