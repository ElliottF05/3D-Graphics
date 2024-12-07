#include "material.h"
#include "light.h"

// MATERIAL ABSTRACT CLASS
Material::~Material() = default;


// LAMBERTIAN MATERIAL
Lambertian::Lambertian(const Vec3& albedo) : albedo(albedo) {}

bool Lambertian::scatter(const Ray &rayIn, const HitRecord &hitRecord, Vec3 &attenuation, Ray &rayOut) const {
    Vec3 scatterDirection = hitRecord.normal + Vec3::randomUnitVector();

    if (scatterDirection.nearZero()) {
        scatterDirection = hitRecord.normal;
    }

    rayOut = Ray(hitRecord.pos, scatterDirection);
    attenuation = albedo;
    return true;
}


// METAL MATERIAL
Metal::Metal(const Vec3& albedo, float fuzziness) : albedo(albedo), fuzziness(fuzziness) {}
Metal::Metal(const Vec3& albedo) : albedo(albedo), fuzziness(0) {}

bool Metal::scatter(const Ray &rayIn, const HitRecord &hitRecord, Vec3 &attenuation, Ray &rayOut) const {
    Vec3 reflected = Vec3::reflect(rayIn.getDirection(), hitRecord.normal);
    reflected = reflected.normalized() + fuzziness * Vec3::randomUnitVector();
    rayOut = Ray(hitRecord.pos, reflected);
    attenuation = albedo;
    return (dot(rayOut.getDirection(), hitRecord.normal) > 0);
}


// DIELECTRIC MATERIAL
Dielectric::Dielectric(float refractionIndex) : refractionIndex(refractionIndex), color(1.0f, 1.0f, 1.0f) {}
Dielectric::Dielectric(const Vec3& color, float refractionIndex) : color(color), refractionIndex(refractionIndex) {}

bool Dielectric::scatter(const Ray &rayIn, const HitRecord &hitRecord, Vec3 &attenuation, Ray &rayOut) const {
    attenuation = color;

    float n1,n2;
    if (hitRecord.frontFace) {
        n1 = 1.0;
        n2 = refractionIndex;
    } else {
        n1 = refractionIndex;
        n2 = 1.0;
    }

    Vec3 rayInUnitDir = rayIn.getDirection().normalized();

    float cosTheta = std::fmin((-1 * rayInUnitDir).dot(hitRecord.normal), 1.0f);
    float sinTheta = std::sqrt(1.0f - cosTheta * cosTheta);

    bool totalInternalReflection = n1 / n2 * sinTheta > 1.0f;
    bool reflectAnyways = reflectance(cosTheta, n1, n2) > randomFloat();

    Vec3 rayOutDir;

    if (totalInternalReflection || reflectAnyways) {
        rayOutDir = Vec3::reflect(rayInUnitDir, hitRecord.normal);
    } else { // can refract OR reflect
        rayOutDir = Vec3::refract(rayInUnitDir, hitRecord.normal, n1, n2);
    }

    Vec3 refracted = Vec3::refract(rayInUnitDir, hitRecord.normal, n1, n2);

    rayOut = Ray(hitRecord.pos, rayOutDir);
    return true;
}

float Dielectric::reflectance(float cosTheta, float n1, float n2) {
    // Uses Schlick's approximation to reflectance (probability of reflecting vs refracting)
    float r0 = (n1 - n2) / (n1 + n2);
    r0 *= r0;
    return r0 + (1 - r0) * std::pow(1 - cosTheta, 5);
}
