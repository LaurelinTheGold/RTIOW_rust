use crate::vec3::*;

#[derive(Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub fn new_dfl() -> Self {
        Ray::new(Point3::new_dfl(), Vec3::new_singleton(1.0))
    }

    /// Get a reference to the ray's dir.
    pub fn dir(&self) -> Vec3 {
        self.dir
    }

    /// Get a reference to the ray's orig.
    pub fn orig(&self) -> Point3 {
        self.orig
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig() + t * self.dir()
    }
}
