use std::rc::Rc;

use crate::hit::Hittable;
use crate::hit::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat_ptr,
        }
    }

    /// Get a reference to the sphere's center.
    pub fn center(&self) -> &Point3 {
        &self.center
    }
    /// Get a reference to the sphere's radius.
    pub fn radius(&self) -> &f64 {
        &self.radius
    }

    /// Get a reference to the sphere's mat ptr.
    pub fn mat_ptr(&self) -> &Rc<dyn Material> {
        &self.mat_ptr
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.orig() - *self.center();
        let a = r.dir().length_squared();
        let half_b = oc.dot(r.dir());
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = half_b * half_b - (a * c);
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        // find nearest acceptable root
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }
        rec.set_t(root);
        rec.set_p(r.at(*rec.t()));
        let outward_normal = (*rec.p() - *self.center()) / *self.radius();
        rec.set_face_normal(r, &outward_normal);
        let tmp = self.mat_ptr().clone();
        rec.set_mat_ptr(tmp);
        // rec.set_normal((*rec.p() - *self.center()) / *self.radius());
        return true;
    }
}
