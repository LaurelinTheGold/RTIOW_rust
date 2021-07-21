use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone, Copy)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64, front_face: bool) -> Self {
        Self {
            p,
            normal,
            t,
            front_face,
        }
    }
    pub fn new_dfl() -> Self {
        Self {
            p: Point3::new_dfl(),
            normal: Vec3::new_dfl(),
            t: 0.0,
            front_face: false,
        }
    }

    /// Set the hit record's t.
    pub fn set_t(&mut self, t: f64) {
        self.t = t;
    }

    /// Set the hit record's p.
    pub fn set_p(&mut self, p: Point3) {
        self.p = p;
    }

    /// Get a reference to the hit record's t.
    pub fn t(&self) -> &f64 {
        &self.t
    }

    /// Get a reference to the hit record's p.
    pub fn p(&self) -> &Point3 {
        &self.p
    }

    /// Set the hit record's normal.
    pub fn set_normal(&mut self, normal: Vec3) {
        self.normal = normal;
    }

    /// Set the hit record's front face.
    pub fn set_front_face(&mut self, front_face: bool) {
        self.front_face = front_face;
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.set_front_face(r.dir().dot(*outward_normal) < 0.0);
        self.set_normal(if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        });
    }

    /// Get a reference to the hit record's normal.
    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
