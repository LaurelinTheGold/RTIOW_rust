use crate::vec3::{Point, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    // pub fn new_dfl() -> Self {
    //     Ray::new(Point::new_dfl(), Vec3::new_singleton(1.0))
    // }

    /// Get a reference to the ray's dir.
    pub fn dir(&self) -> Vec3 {
        self.dir
    }

    /// Get a reference to the ray's orig.
    pub fn orig(&self) -> Point {
        self.orig
    }

    pub fn at(&self, t: f32) -> Point {
        self.orig() + t * self.dir()
    }
}
