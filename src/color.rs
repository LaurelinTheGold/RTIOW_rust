use crate::vec3::Color;

const SCALE_FACTOR: f64 = 255.999;

pub fn write_color(pixel_color: Color) {
    println!(
        "{} {} {}",
        (pixel_color.x() * SCALE_FACTOR) as u64,
        (pixel_color.y() * SCALE_FACTOR) as u64,
        (pixel_color.z() * SCALE_FACTOR) as u64
    )
}
