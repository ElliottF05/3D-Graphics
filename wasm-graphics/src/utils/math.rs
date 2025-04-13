use std::{f32::consts::PI, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}};

use super::utils::random_float;

pub const fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * PI / 180.0;
}
pub const fn radians_to_degrees(radians: f32) -> f32 {
    return radians * 180.0 / PI;
}

// Vec3 struct, basid 3d vector geometry
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {

    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        return Vec3{x,y,z};
    }
    #[inline(always)]
    pub fn zero() -> Self {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    /// Returns a vector with each component in range [0, 1.0]
    #[inline(always)]
    pub fn random() -> Self {
        return Vec3::new(
            random_float(),
            random_float(),
            random_float()
        );
    }

    #[inline(always)]
    pub fn random_range(min: f32, max: f32) -> Self {
        return Vec3::random() * (max - min) + Vec3::new(min, min, min);
    }
    #[inline(always)]
    pub fn random_on_unit_sphere() -> Self {
        loop {
            let v = Vec3::random_range(-1.0, 1.0);
            let len_squared = v.len_squared();
            if 1e-30 < len_squared && len_squared < 1.0 {
                return v / len_squared.sqrt();
            }
        }
    }
    #[inline(always)]
    pub fn random_on_hemisphere(normal: Vec3) -> Self {
        let mut v = Vec3::random_on_unit_sphere();
        if v.dot(normal) < 0.0 {
            v = -v;
        }
        return v;
    }

    #[inline(always)]
    pub fn dot_product(a: Self, b: Self) -> f32 {
        return a.x * b.x + a.y * b.y + a.z * b.z;
    }
    #[inline(always)]
    pub fn dot(self, other: Self) -> f32 {
        return Self::dot_product(self, other);
    }
    #[inline(always)]
    pub fn cross_product(a: Self, b: Self) -> Self {
        return Self::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x
        );
    }
    #[inline(always)]
    pub fn cross(self, other: Self) -> Self {
        return Self::cross_product(self, other);
    }
    #[inline(always)]
    pub fn len(self) -> f32 {
        return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    }
    #[inline(always)]
    pub fn len_squared(self) -> f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
    #[inline(always)]
    pub fn normalize(&mut self) {
        *self /= self.len();
    }
    #[inline(always)]
    pub fn normalized(self) -> Self {
        return self / self.len();
    }

    #[inline(always)]
    pub fn rotate_z(&mut self, theta_z: f32) {
        let (sin, cos) = theta_z.sin_cos();
        let (x, y) = (self.x, self.y);
        self.x = x * cos - y * sin;
        self.y = x * sin + y * cos;
    }
    #[inline(always)]
    pub fn rotate_y(&mut self, theta_y: f32) {
        let (sin, cos) = theta_y.sin_cos();
        let (x, z) = (self.x, self.z);
        self.x = x * cos - z * sin;
        self.z = x * sin + z * cos;
    }
    #[inline(always)]
    pub fn rotate_z_fast(&mut self, sin_z: f32, cos_z: f32) {
        let (x, y) = (self.x, self.y);
        self.x = x * cos_z - y * sin_z;
        self.y = x * sin_z + y * cos_z;
    }
    #[inline(always)]
    pub fn rotate_y_fast(&mut self, sin_y: f32, cos_y: f32) {
        let (x, z) = (self.x, self.z);
        self.x = x * cos_y - z * sin_y;
        self.z = x * sin_y + z * cos_y;
    }

    #[inline(always)]
    pub fn rotated_z(self, theta_z: f32) -> Vec3 {
        let (sin, cos) = theta_z.sin_cos();
        return Vec3 { 
            x: self.x * cos - self.y * sin, 
            y: self.x * sin + self.y * cos, 
            z: self.z 
        };
    }
    #[inline(always)]
    pub fn rotated_y(self, theta_y: f32) -> Vec3 {
        let (sin, cos) = theta_y.sin_cos();
        return Vec3 { 
            x: self.x * cos - self.z * sin, 
            y: self.y, 
            z: self.x * sin + self.z * cos 
        };
    }
    #[inline(always)]
    pub fn rotated_z_fast(self, sin_z: f32, cos_z: f32) -> Vec3 {
        return Vec3 {
            x: self.x * cos_z - self.y * sin_z,
            y: self.x * sin_z + self.y * cos_z,
            z: self.z,
        }
    }
    #[inline(always)]
    pub fn rotated_y_fast(self, sin_y: f32, cos_y: f32) -> Vec3 {
        return Vec3 {
            x: self.x * cos_y - self.z * sin_y,
            y: self.y,
            z: self.x * sin_y + self.z * cos_y,
        }
    }

    #[inline(always)]
    pub fn mul_elementwise(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
    #[inline(always)]
    pub fn mul_elementwise_inplace(&mut self, other: Vec3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
    #[inline(always)]
    pub fn mul_elementwise_of(a: Vec3, b: Vec3) -> Vec3 {
        Vec3::new(a.x * b.x, a.y * b.y, a.z * b.z)
    }

    #[inline(always)]
    pub fn midpoint_with(self, other: Vec3) -> Vec3 {
        return 0.5 * (self + other);
    }
    #[inline(always)]
    pub fn midpoint_of(a: Vec3, b: Vec3) -> Vec3 {
        return a.midpoint_with(b);
    }

    #[inline(always)]
    pub fn clamp(&mut self, min: f32, max: f32) {
        self.x = self.x.clamp(min, max);
        self.y = self.y.clamp(min, max);
        self.z = self.z.clamp(min, max);
    }
    #[inline(always)]
    pub fn near_zero(self) -> bool {
        const EPSILON: f32 = 1e-8;
        return self.x.abs() < EPSILON && self.y.abs() < EPSILON && self.z.abs() < EPSILON;
    }
    #[inline(always)]
    pub fn min_elementwise(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z)
        )
    }
    #[inline(always)]
    pub fn max_elementwise(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z)
        )
    }

    #[inline(always)]
    pub fn reflect(self, normal: Vec3) -> Vec3 {
        return self - 2.0 * self.dot(normal) * normal;
    }
    #[inline(always)]
    pub fn refract(self, normal: Vec3, n1_over_n2: f32) -> Vec3 {
        let incoming_dir = self;
        let mut cos_theta = -incoming_dir.dot(normal);
        cos_theta = cos_theta.min(1.0);

        let outgoing_perp = n1_over_n2 * (incoming_dir + cos_theta * normal);
        let outgoing_parallel = -(1.0 - outgoing_perp.len_squared()).abs().sqrt() * normal;

        return outgoing_perp + outgoing_parallel;
    }

}

impl Add for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Neg for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl Mul<f32> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: vec.x * self,
            y: vec.y * self,
            z: vec.z * self,
        }
    }
}
impl Div<f32> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn div(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}
impl AddAssign for Vec3 {
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
impl SubAssign for Vec3 {
    #[inline(always)]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}
impl MulAssign<f32> for Vec3 {
    #[inline(always)]
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}
impl DivAssign<f32> for Vec3 {
    #[inline(always)]
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}