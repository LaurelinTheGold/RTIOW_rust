use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use rand::Rng;

use crate::utils::{random, random_range};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

pub type Color = Vec3;
pub type Point = Vec3;

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub const fn new_singleton(e: f32) -> Self {
        Self::new(e, e, e)
    }
    pub const fn new_dfl() -> Self {
        Self::new_singleton(0_f32)
    }

    /// Get vec3's x.
    pub const fn x(&self) -> f32 {
        self.x
    }

    /// Get vec3's y.
    pub const fn y(&self) -> f32 {
        self.y
    }

    /// Get vec3's z.
    pub const fn z(&self) -> f32 {
        self.z
    }
}
impl Vec3 {
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(self) -> f32 {
        let a = self.x();
        let b = self.y();
        let c = self.z();
        a * a + b * b + c * c
    }

    pub fn dot(self, other: Vec3) -> f32 {
        let tmp = self * other;
        tmp.x() + tmp.y() + tmp.z()
    }
    pub fn cross(self, other: Vec3) -> Vec3 {
        Self {
            x: self.y() * other.z() - self.z() * other.y(),
            y: self.z() * other.x() - self.x() * other.z(),
            z: self.x() * other.y() - self.y() * other.x(),
        }
    }
    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }
    pub fn near_zero(self) -> bool {
        let s = 1e-8_f32;
        self.x().abs() < s && self.y().abs() < s && self.z().abs() < s
    }
    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }
    pub fn refract(self, n: Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = n.dot(-self).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = (1.0 - r_out_perp.length_squared()).abs().sqrt() * -1.0 * n;
        r_out_parallel + r_out_perp
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs
    }
}
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs
    }
}
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1_f32 / rhs
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: (self.x() + rhs.x()),
            y: (self.y() + rhs.y()),
            z: (self.z() + rhs.z()),
        }
    }
}
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {
            x: (self.x() * rhs.x()),
            y: (self.y() * rhs.y()),
            z: (self.z() * rhs.z()),
        }
    }
}
impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        self * Vec3::new_singleton(rhs)
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}
impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        self * (1_f32 / rhs)
    }
}

impl Vec3 {
    pub fn random_vec3<R: Rng + ?Sized>(rng: &mut R) -> Vec3 {
        Vec3::new(random(rng), random(rng), random(rng))
    }
    pub fn random_vec3_range<R: Rng + ?Sized>(min: f32, max: f32, rng: &mut R) -> Vec3 {
        Vec3::new(
            random_range(min, max, rng),
            random_range(min, max, rng),
            random_range(min, max, rng),
        )
    }
    /// Initial Lambertian
    pub fn random_in_unit_sphere<R: Rng + ?Sized>(rng: &mut R) -> Vec3 {
        let mut out: Vec3 = Vec3::random_vec3_range(-1.0, 1.0, rng);
        loop {
            if out.length_squared() < 1.0 {
                return out;
            }
            out = Vec3::random_vec3_range(-1.0, 1.0, rng);
        }
    }
    /// Better Lambertian
    pub fn random_unit_vector<R: Rng + ?Sized>(rng: &mut R) -> Vec3 {
        Vec3::random_in_unit_sphere(rng).unit_vector()
    }
    /// Even better Lambertian
    pub fn random_in_hemisphere<R: Rng + ?Sized>(normal: Vec3, rng: &mut R) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere(rng);
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
    pub fn random_in_unit_disk<R: Rng + ?Sized>(rng: &mut R) -> Vec3 {
        loop {
            let p = Vec3::new(
                random_range(-1.0, 1.0, rng),
                random_range(-1.0, 1.0, rng),
                0.0,
            );
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
}

#[test]
fn test_linalg() {
    let vecx = Vec3::new(1.0, 0.0, 0.0);
    let vecy = Vec3::new(0.0, 1.0, 0.0);
    let vecz = Vec3::new(0.0, 0.0, 1.0);
    let zero = Vec3::new_dfl();
    assert_eq!(vecz, Vec3::cross(vecx, vecy));
    assert_eq!(-vecz, Vec3::cross(vecy, vecx));
    assert_eq!(zero, vecx.cross(vecx));
    assert_eq!(zero, vecy.cross(vecy));
    assert_eq!(zero, vecz.cross(vecz));
    assert_eq!(0.0, vecx.dot(vecy));
    assert_eq!(1.0, vecx.dot(vecx));
}
