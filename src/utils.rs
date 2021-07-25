use std::f32::consts::PI;

use rand::Rng;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random<R: Rng + ?Sized>(rng: &mut R) -> f32 {
    rng.gen()
}

pub fn random_range<R: Rng + ?Sized>(min: f32, max: f32, rng: &mut R) -> f32 {
    min + random(rng) * (max - min)
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    // Schlick Approximation
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
