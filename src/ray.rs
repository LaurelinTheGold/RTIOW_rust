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
