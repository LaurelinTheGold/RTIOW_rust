use crate::{
    hit::HitRecord,
    ray::Ray,
    utils::random_double,
    vec3::{Color, Vec3},
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive()]
pub struct Lambertian {
    albedo: Color,
}
impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    /// Get a reference to the lambertian's albedo.
    pub fn albedo(&self) -> &Color {
        &self.albedo
    }
}
impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = *rec.normal() + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = *rec.normal();
        }
        *scattered = Ray::new(*rec.p(), scatter_direction);
        *attenuation = *self.albedo();
        true
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }

    /// Get a reference to the metal's albedo.
    pub fn albedo(&self) -> &Color {
        &self.albedo
    }

    /// Get a reference to the metal's fuzz.
    pub fn fuzz(&self) -> &f64 {
        &self.fuzz
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = r_in.dir().unit_vector().reflect(rec.normal());
        *scattered = Ray::new(
            *rec.p(),
            reflected + *self.fuzz() * Vec3::random_in_unit_sphere(),
        );
        *attenuation = *self.albedo();
        scattered.dir().dot(*rec.normal()) > 0.0
    }
}

pub struct Dielectric {
    ir: f64,
}
impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    /// Get a reference to the dielectric's ir.
    pub fn ir(&self) -> &f64 {
        &self.ir
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Schlick Approximation
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new_singleton(1.0);
        let refraction_ratio = if *rec.front_face() {
            1.0 / self.ir()
        } else {
            *self.ir()
        };
        let unit_direction = r_in.dir().unit_vector();
        let cos_theta = -unit_direction.dot(*rec.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double()
        {
            unit_direction.reflect(rec.normal())
        } else {
            Vec3::refract(&unit_direction, rec.normal(), refraction_ratio)
        };
        *scattered = Ray::new(*rec.p(), direction);
        true
    }
}
