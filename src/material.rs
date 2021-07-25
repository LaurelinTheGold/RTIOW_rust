use rand::Rng;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    utils::{random, schlick},
    vec3::{Color, Vec3},
};

#[derive(Clone, Copy, Debug)]
pub struct ScatterBundle {
    albedo: Color,
    ray: Ray,
}

impl ScatterBundle {
    pub fn new(albedo: Color, ray: Ray) -> Self {
        Self { albedo, ray }
    }

    /// Get the scatter bundle's albedo.
    pub fn albedo(&self) -> Color {
        self.albedo
    }

    /// Get the scatter bundle's ray.
    pub fn ray(&self) -> Ray {
        self.ray
    }
}
pub trait Material {
    fn scatter<R: Rng + ?Sized>(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        rng: &mut R,
    ) -> Option<ScatterBundle>;
}
pub enum MaterialType {
    Lambertian(Vec3),
    Metal(Vec3, f32),
    Dielectric(f32),
}

impl MaterialType {}
impl Material for MaterialType {
    fn scatter<R: Rng + ?Sized>(
        &self,
        r_in: Ray,
        rec: &HitRecord,
        rng: &mut R,
    ) -> Option<ScatterBundle> {
        match self {
            MaterialType::Lambertian(albedo) => {
                let scatter_dir = rec.normal() + Vec3::random_unit_vector(rng);
                match scatter_dir.near_zero() {
                    true => Some(ScatterBundle::new(*albedo, Ray::new(rec.p(), rec.normal()))),
                    false => Some(ScatterBundle::new(*albedo, Ray::new(rec.p(), scatter_dir))),
                }
            }
            MaterialType::Metal(albedo, fuzz) => {
                let reflected = r_in.dir().unit_vector().reflect(rec.normal());
                let scattered = Ray::new(
                    rec.p(),
                    reflected + *fuzz * Vec3::random_in_unit_sphere(rng),
                );
                match scattered.dir().dot(rec.normal()) > 0.0 {
                    true => Some(ScatterBundle::new(*albedo, scattered)),
                    false => None,
                }
            }
            MaterialType::Dielectric(ir) => {
                let refrac_ratio = if rec.front_face() { 1.0 / ir } else { *ir };
                let unit_dir = r_in.dir().unit_vector();
                let cos = (-1. * unit_dir).dot(rec.normal()).min(1.);
                let cannot_refract = refrac_ratio * (1. - cos * cos).sqrt() > 1.;
                Some(ScatterBundle::new(
                    Color::new_singleton(1.0),
                    Ray::new(
                        rec.p(),
                        match cannot_refract || schlick(cos, refrac_ratio) > random(rng) {
                            true => unit_dir.reflect(rec.normal()),
                            false => Vec3::refract(unit_dir, rec.normal(), refrac_ratio),
                        },
                    ),
                ))
            }
        }
    }
}
