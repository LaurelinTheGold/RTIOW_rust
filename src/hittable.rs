// use std::rc::Rc;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{material::MaterialType, ray::Ray, vec3::Vec3};

pub struct HitRecord<'a> {
    p: Vec3,
    t: f32,
    normal: Vec3,
    mat_ptr: &'a MaterialType,
    front_face: bool,
}

impl<'a> HitRecord<'a> {
    /// Get the hit record's normal.
    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    /// Get the hit record's p.
    pub fn p(&self) -> Vec3 {
        self.p
    }

    /// Get if the hit record is a front face.
    pub fn front_face(&self) -> bool {
        self.front_face
    }

    /// Get the hit record's t.
    pub fn t(&self) -> f32 {
        self.t
    }

    /// Get the hit record's mat_ptr.
    pub fn mat_ptr(&self) -> &'a MaterialType {
        self.mat_ptr
    }
}
pub trait Hittable {
    fn hit(&self, r: Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}
pub enum HittableObject {
    Sphere(Vec3, f32, MaterialType),
    HittableList(Vec<HittableObject>),
}

impl<'b> Hittable for HittableObject {
    fn hit(&self, r: Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        match self {
            HittableObject::HittableList(a) => a
                .par_iter()
                .map(|x| -> Option<HitRecord> { x.hit(r, tmin, tmax) })
                .reduce(
                    || None,
                    |boi, food| match food {
                        Some(hit) => match boi {
                            Some(prev_hit) => {
                                if prev_hit.t() > hit.t() {
                                    Some(hit)
                                } else {
                                    Some(prev_hit)
                                }
                            }
                            None => Some(hit),
                        },
                        None => boi,
                    },
                ),
            // .fold(None, |boi, food| match food {
            //     Some(hit) => match boi {
            //         Some(prev_hit) => {
            //             if prev_hit.t() > hit.t() {
            //                 Some(hit)
            //             } else {
            //                 Some(prev_hit)
            //             }
            //         }
            //         None => Some(hit),
            //     },
            //     None => boi,
            // }),
            HittableObject::Sphere(center, radius, mat_ptr) => {
                let oc = r.orig() - *center;
                let a = r.dir().length_squared();
                let half_b = oc.dot(r.dir());
                let c = oc.length_squared() - (*radius * *radius);

                let discr = half_b * half_b - (a * c);
                match discr {
                    d if d < 0.0 => None,
                    d => {
                        let sqrtd = d.sqrt();
                        let mut root = (-half_b - sqrtd) / a;
                        if root < tmin || root > tmax {
                            root = (-half_b + sqrtd) / a;
                            if root < tmin || root > tmax {
                                return None;
                            }
                        }
                        let t = root;
                        let p = r.at(t);
                        let out_norm = (p - *center) / *radius;
                        let front = r.dir().dot(out_norm) < 0.0;
                        let norm = if front { out_norm } else { -out_norm };
                        Some(HitRecord {
                            p,
                            t,
                            normal: norm,
                            mat_ptr,
                            front_face: (front),
                        })
                    }
                }
            }
        }
    }
}
