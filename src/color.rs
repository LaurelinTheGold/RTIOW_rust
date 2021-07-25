use crate::vec3::Color;

const SCALE_FACTOR: f32 = 255.999;

pub fn write_color(pixel_color: Color, samples_per_pixel: usize) {
    let scale = 1.0 / samples_per_pixel as f32;
    const ZERO: f32 = 0.0;
    const ONEISH: f32 = 0.999;

    println!(
        "{} {} {}",
        (SCALE_FACTOR * (pixel_color.x() * scale).sqrt().clamp(ZERO, ONEISH)) as u64,
        (SCALE_FACTOR * (pixel_color.y() * scale).sqrt().clamp(ZERO, ONEISH)) as u64,
        (SCALE_FACTOR * (pixel_color.z() * scale).sqrt().clamp(ZERO, ONEISH)) as u64
    )
}

fn write_color_bitboi(c: u32) {
    println!("{} {} {}", c >> 16 & 0xFF, c >> 8 & 0xFF, c & 0xFF)
}

pub fn print_output(pic: Vec<Vec<u32>>, width: usize, height: usize, colorsize: usize) {
    println!("P3\n{} {}\n{}", width, height, colorsize - 1);
    for row in pic.iter() {
        for col in row.iter() {
            write_color_bitboi(*col);
        }
    }
}
