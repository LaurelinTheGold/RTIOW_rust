use crate::vec3::Color;

const SCALE_FACTOR: f64 = 255.999;

pub fn write_color(pixel_color: Color, samples_per_pixel: usize) {
    let scale = 1.0 / samples_per_pixel as f64;
    const ZERO: f64 = 0.0;
    const ONEISH: f64 = 0.999;

    println!(
        "{} {} {}",
        (SCALE_FACTOR * (pixel_color.r() * scale).sqrt().clamp(ZERO, ONEISH)) as u64,
        (SCALE_FACTOR * (pixel_color.g() * scale).sqrt().clamp(ZERO, ONEISH)) as u64,
        (SCALE_FACTOR * (pixel_color.b() * scale).sqrt().clamp(ZERO, ONEISH)) as u64
    )
}
