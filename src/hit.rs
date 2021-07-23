use std::{fmt::Debug, rc::Rc};

use crate::{
    material::{Lambertian, Material},
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    mat_ptr: Rc<dyn Material>,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        normal: Vec3,
        mat_ptr: Rc<dyn Material>,
        t: f64,
        front_face: bool,
    ) -> Self {
        Self {
            p,
            normal,
            mat_ptr,
            t,
            front_face,
        }
    }

    pub fn new_dfl() -> Self {
        Self::new(
            Point3::new_dfl(),
            Vec3::new_dfl(),
            Rc::new(Lambertian::new(Vec3::new_singleton(0.5))), // Change later
            0.0,
            false,
        )
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

    /// Set the hit record's mat ptr.
    pub fn set_mat_ptr(&mut self, mat_ptr: Rc<dyn Material>) {
        self.mat_ptr = mat_ptr;
    }

    /// Get a reference to the hit record's mat ptr.
    pub fn mat_ptr(&self) -> &Rc<dyn Material> {
        &self.mat_ptr
    }

    /// Get a reference to the hit record's front face.
    pub fn front_face(&self) -> &bool {
        &self.front_face
    }
}
impl Debug for HitRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(
        //     f,
        //     "p: {:?}, n: {:?}, t: {:?}, front: {:?}",
        //     self.p, self.normal, self.t, self.front_face
        // )
        write!(f, "p: {:?}, n: {:?}", self.p, self.normal)
    }
}
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
