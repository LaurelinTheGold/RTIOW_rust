use std::{convert::TryInto, ops::*};

pub(crate) type Color = Vec3;
pub(crate) type Point3 = Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    elem: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { elem: [e0, e1, e2] }
    }
    pub fn new_singleton(e: f64) -> Self {
        Self::new(e, e, e)
    }
    pub fn new_dfl() -> Self {
        Self::new_singleton(0_f64)
    }
}
impl Vec3 {
    pub fn x(&self) -> f64 {
        self.elem[0]
    }
    pub fn y(&self) -> f64 {
        self.elem[1]
    }
    pub fn z(&self) -> f64 {
        self.elem[2]
    }
    pub fn r(&self) -> f64 {
        self.elem[0]
    }
    pub fn g(&self) -> f64 {
        self.elem[1]
    }
    pub fn b(&self) -> f64 {
        self.elem[2]
    }
    pub fn print(&self) -> () {
        println!("{:?}", self)
    }
    pub fn length(&self) -> f64 {
        self.elem.iter().map(|x| (*x) * (*x)).sum::<f64>().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        let tmp = self.length();
        tmp * tmp
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Self {
            elem: self
                .elem
                .iter()
                .map(|x| -x)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}
impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.elem[index]
    }
}
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elem[index]
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.elem
            .iter_mut()
            .zip(rhs.elem.iter())
            .for_each(|(x, y)| *x += y);
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs
    }
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.elem.iter_mut().for_each(|x| *x *= rhs);
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1_f64 / rhs
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            elem: self
                .elem
                .iter()
                .zip(rhs.elem.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
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
            elem: self
                .elem
                .iter()
                .zip(rhs.elem.iter())
                .map(|(a, b)| a * b)
                .collect::<Vec<f64>>()
                .try_into()
                .unwrap(),
        }
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        self * Vec3::new_singleton(rhs)
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1_f64 / rhs)
    }
}

impl Vec3 {
    pub fn dot(&self, other: Vec3) -> f64 {
        (*self * other).elem.iter().sum::<f64>()
    }
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Self {
            elem: [
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(),
            ],
        }
    }
    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }
}

#[test]
fn test_gen() {
    let vec: Vec3 = Vec3::new(0.1, 0.2, 0.3);
    vec.print();
}
#[test]
fn test_xyz() {
    let vec: Vec3 = Vec3::new(0.1, 0.2, 0.3);
    assert_eq!(vec.x(), 0.1);
    assert_eq!(vec.y(), 0.2);
    assert_eq!(vec.z(), 0.3);
}
#[test]
fn test_rgb() {
    let vec: Vec3 = Vec3::new(0.1, 0.2, 0.3);
    assert_eq!(vec.r(), 0.1);
    assert_eq!(vec.g(), 0.2);
    assert_eq!(vec.b(), 0.3);
}
#[test]
fn test_neg() {
    let vec: Vec3 = Vec3::new(0.1, 0.2, 0.3);
    assert_eq!(Vec3::new(-0.1, -0.2, -0.3), -vec)
}
#[test]
fn test_idx() {
    let vec: Vec3 = Vec3::new(0.1, 0.2, 0.3);
    assert_eq!(vec[0], 0.1);
    assert_eq!(vec[1], 0.2);
    assert_eq!(vec[2], 0.3);
}
#[test]
fn test_addeq() {
    let mut vec: Vec3 = Vec3::new(0.1, 0.2, 0.3);
    // let vec2 = vec.clone();
    vec += vec;
    assert_eq!(Vec3::new(0.2, 0.4, 0.6), vec);
    vec += vec;
    assert_eq!(Vec3::new(0.4, 0.8, 1.2), vec);
}
#[test]
fn test_subeq() {
    let mut vec = Vec3::new(0.1, 0.2, 0.3);
    vec -= vec;
    assert_eq!(Vec3::new_dfl(), vec)
}

#[test]
fn test_muleq() {
    let mut vec = Vec3::new(0.1, 0.2, 0.3);
    vec *= 2.0;
    assert_eq!(Vec3::new(0.2, 0.4, 0.6), vec);
    vec *= 0.0;
    assert_eq!(Vec3::new_dfl(), vec);
}

#[test]
fn test_diveq() {
    use std::f64::INFINITY;
    let mut vec = Vec3::new(0.1, 0.2, 0.3);
    vec /= 2.0;
    assert_eq!(Vec3::new(0.05, 0.1, 0.15), vec);
    vec /= 0.0;
    assert_eq!(Vec3::new(INFINITY, INFINITY, INFINITY), vec)
}
#[test]
fn test_length() {
    assert_eq!(0.0, Vec3::new_dfl().length());
    assert_eq!(14.0f64.sqrt(), Vec3::new(1.0, 2.0, 3.0).length());
}
#[test]
fn test_length_squared() {
    assert_eq!(0.0, Vec3::new_dfl().length());
    assert_eq!(14.0f64, Vec3::new(1.0, 2.0, 3.0).length_squared());
}
#[test]
fn test_add() {
    let vec: Vec3 = Vec3::new(0.1, 0.2, 0.3);
    assert_eq!(Vec3::new(0.2, 0.4, 0.6), vec + vec)
}
#[test]
fn test_sub() {
    let vec: Vec3 = Vec3::new(0.1, 0.2, 0.3);
    assert_eq!(Vec3::new_dfl(), vec - vec)
}
#[test]
fn test_mul() {
    let vec: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    let zero = 0.0_f64;
    let two = 2_f64;
    assert_eq!(Vec3::new(1.0, 4.0, 9.0), vec * vec);
    assert_eq!(Vec3::new_dfl(), vec * zero);
    assert_eq!(Vec3::new(2.0, 4.0, 6.0), vec * two);
    assert_eq!(Vec3::new_dfl(), zero * vec);
    assert_eq!(Vec3::new(2.0, 4.0, 6.0), two * vec);
}
#[test]
fn test_div() {
    use std::f64::INFINITY;
    let vec: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    let zero = 0.0_f64;
    let two = 2_f64;
    assert_eq!(Vec3::new(0.5, 1.0, 1.5), vec / two);
    assert_eq!(Vec3::new(INFINITY, INFINITY, INFINITY), vec / zero);
}
#[test]
fn test_dot() {
    let vec: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    let zero = 0.0_f64;
    let zero_vec = Vec3::new_dfl();
    assert_eq!(14_f64, vec.dot(vec));
    assert_eq!(zero, vec.dot(zero_vec));
    assert_eq!(zero, zero_vec.dot(vec));
}

#[test]
fn test_cross() {
    let vec1 = Vec3::new(2.0, 4.0, 5.0);
    let vec2 = Vec3::new(-3.0, 2.0, 3.0);
    assert_eq!(Vec3::new(2.0, -21.0, 16.0), vec1.cross(vec2))
}

#[test]
fn test_unitvector() {
    let vec = Vec3::new(2.0, 0.0, 0.0);
    assert_eq!(Vec3::new(1.0, 0.0, 0.0), vec.unit_vector());
    // Do not know how to do float equals but just fp error off
    // let vec = Vec3::new(3.0, 4.0, 0.0);
    // assert_eq!(Vec3::new(0.6, 0.8, 0.0), vec.unit_vector())
}

#[test]
fn test_linalg() {
    let vecx = Vec3::new(1.0, 0.0, 0.0);
    let vecy = Vec3::new(0.0, 1.0, 0.0);
    let vecz = Vec3::new(0.0, 0.0, 1.0);
    let zero = Vec3::new_dfl();
    assert_eq!(vecz, Vec3::cross(&vecx, vecy));
    assert_eq!(-vecz, Vec3::cross(&vecy, vecx));
    assert_eq!(zero, vecx.cross(vecx));
    assert_eq!(zero, vecy.cross(vecy));
    assert_eq!(zero, vecz.cross(vecz));
    assert_eq!(0.0, vecx.dot(vecy));
    assert_eq!(1.0, vecx.dot(vecx));
}
