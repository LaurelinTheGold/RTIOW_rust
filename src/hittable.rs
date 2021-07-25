// use std::rc::Rc;

use crate::{
    material::MaterialType,
    ray::Ray,
    vec3::{Point, Vec3},
};

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

    /// Set the hit record's t.
    pub fn set_t(&mut self, t: f32) {
        self.t = t;
    }

    /// Set the hit record's p.
    pub fn set_p(&mut self, p: Point) {
        self.p = p;
    }

    /// Get the hit record's t.
    pub fn t(&self) -> f32 {
        self.t
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

    // /// Set the hit record's mat ptr.
    // pub fn set_mat_ptr(&mut self, mat_ptr: Rc<dyn Material>) {
    //     self.mat_ptr = mat_ptr;
    // }

    // /// Get a reference to the hit record's mat ptr.
    // pub fn mat_ptr(&self) -> &Rc<dyn Material> {
    //     &self.mat_ptr
    // }

    /// Get the hit record's mat_ptr.
    pub fn mat_ptr(&self) -> &'a MaterialType {
        self.mat_ptr
    }
}
// impl Debug for HitRecord {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "p: {:?}, n: {:?}", self.p, self.normal)
//     }
// }
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
            HittableObject::HittableList(a) => {
                a.iter()
                    .map(|x| x.hit(r, tmin, tmax))
                    .fold(None, |boi, food| match food {
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
                    })
            }
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
                        let root1 = (-half_b - sqrtd) / a;
                        let root2 = (-half_b + sqrtd) / a;
                        match root1 < tmin || root1 > tmax {
                            false => {
                                let t = root1;
                                let p = r.at(t);
                                let out_norm = (p - *center) / *radius;
                                let front = r.dir().dot(out_norm) < 0.0;
                                let norm = if front { out_norm } else { -out_norm };
                                Some(HitRecord {
                                    p,
                                    t,
                                    normal: norm,
                                    mat_ptr: (*&mat_ptr),
                                    front_face: (front),
                                })
                            }
                            true => match root2 < tmin || root2 > tmax {
                                false => {
                                    let t = root1;
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
                                true => None,
                            },
                        }
                    }
                }
            }
        }
    }
}
// impl Hittable for Sphere {
//     fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
//         let oc = r.orig() - *self.center();
//         let a = r.dir().length_squared();
//         let half_b = oc.dot(r.dir());
//         let c = oc.length_squared() - (self.radius * self.radius);

//         let discriminant = half_b * half_b - (a * c);
//         if discriminant < 0.0 {
//             return false;
//         }
//         let sqrtd = discriminant.sqrt();
//         // find nearest acceptable root
//         let mut root = (-half_b - sqrtd) / a;
//         if root < t_min || root > t_max {
//             root = (-half_b + sqrtd) / a;
//             if root < t_min || root > t_max {
//                 return false;
//             }
//         }
//         rec.set_t(root);
//         rec.set_p(r.at(rec.t()));
//         let outward_normal = (rec.p() - *self.center()) / *self.radius();
//         rec.set_face_normal(r, &outward_normal);
//         let tmp = self.mat_ptr().clone();
//         rec.set_mat_ptr(tmp);
//         // rec.set_normal((*rec.p() - *self.center()) / *self.radius());
//         return true;
//     }
// }

// impl HittableObject {
//     pub fn new_dfl() -> Self {
//         Self {
//             objects: Vec::new(),
//         }
//     }
//     pub fn add(&mut self, object: Rc<dyn Hittable>) {
//         self.objects.push(object);
//     }
// }

// pub struct Sphere<'b> {
//     center: Point,
//     radius: f32,
//     mat_ptr: &'b MaterialType,
// }

// impl Sphere {
//     pub fn new(center: Point, radius: f32, mat_ptr: Rc<dyn Material>) -> Self {
//         Self {
//             center,
//             radius,
//             mat_ptr,
//         }
//     }

//     /// Get a reference to the sphere's center.
//     pub fn center(&self) -> &Point {
//         &self.center
//     }
//     /// Get a reference to the sphere's radius.
//     pub fn radius(&self) -> &f32 {
//         &self.radius
//     }

//     // /// Get a reference to the sphere's mat ptr.
//     // pub fn mat_ptr(&self) -> &Rc<dyn Material> {
//     //     &self.mat_ptr
//     // }
// }
// impl Hittable for HittableObject {
//     fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
//         let mut hit_anything = false;
//         let mut closest_so_far = t_max;
//         let mut rec = rec;
//         for obj in self.objects.iter() {
//             if obj.hit(r, t_min, closest_so_far, &mut rec) {
//                 hit_anything = true;
//                 closest_so_far = rec.t();
//             }
//         }
//         return hit_anything;
//     }
// }
