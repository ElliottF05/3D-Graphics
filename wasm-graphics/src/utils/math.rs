use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        return Vec3{x,y,z};
    }
    pub fn dot_product(a: &Self, b: &Self) -> f32 {
        return a.x * b.x + a.y * b.y + a.z * b.z;
    }
    pub fn dot(&self, other: &Self) -> f32 {
        return Self::dot_product(&self, other);
    }
    pub fn cross_product(a: &Self, b: &Self) -> Self {
        return Self::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x
        );
    }
    pub fn cross(&self, other: &Self) -> Self {
        return Self::cross_product(&self, other);
    }
    pub fn len(&self) -> f32 {
        return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    }
    pub fn len_squared(&self) -> f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
    pub fn normalize(&mut self) {
        *self /= self.len();
    }
    pub fn normalized(&self) -> Self {
        return self.clone() / self.len();
    }

    pub fn rotate_z(&mut self, theta_z: f32) {
        let (sin, cos) = theta_z.sin_cos();
        let (x, y) = (self.x, self.y);
        self.x = x * cos - y * sin;
        self.y = x * sin + y * cos;
    }
    pub fn rotate_y(&mut self, theta_y: f32) {
        let (sin, cos) = theta_y.sin_cos();
        let (x, z) = (self.x, self.z);
        self.x = x * cos - z * sin;
        self.z = x * sin + z * cos;
    }
    pub fn rotate_z_fast(&mut self, sin_z: f32, cos_z: f32) {
        let (x, y) = (self.x, self.y);
        self.x = x * cos_z - y * sin_z;
        self.y = x * sin_z + y * cos_z;
    }
    pub fn rotate_y_fast(&mut self, sin_y: f32, cos_y: f32) {
        let (x, z) = (self.x, self.z);
        self.x = x * cos_y - z * sin_y;
        self.z = x * sin_y + z * cos_y;
    }


    pub fn pairwise_mul(&mut self, other: &Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
    pub fn pairwise_mul_new(a: &Vec3, b: &Vec3) -> Vec3 {
        return Vec3::new(a.x * b.x, a.y * b.y, a.z * b.z);
    }
}

impl Add for Vec3 {
    type Output = Vec3;
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

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Mul<f32> for Vec3 {
    type Output = Vec3;
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
    fn div(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}