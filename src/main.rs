use std::io::{self, Write};

use ray::Ray;
use vec3::Vec3;

use crate::{color::write_color, vec3::*};

mod color;
mod hit;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    use vec3::*;
    let oc: Vec3 = r.orig() - *center;
    // let a = Vec3::dot(&r.dir(), r.dir());
    // let b = 2.0 * Vec3::dot(&oc, r.dir());
    // let c = Vec3::dot(&oc, oc) - radius * radius;
    // let discriminant = b * b - 4.0 * a * c;
    // discriminant > 0.0
    let a = r.dir().length_squared();
    let half_b = oc.dot(r.dir());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / (a)
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = Vec3::unit_vector(&(r.at(t) - Vec3::new(0.0, 0.0, -1.0)));
        return 0.5 * Color::new(n.r() + 1.0, n.g() + 1.0, n.b() + 1.0);
    }
    let unit_direction: Vec3 = r.dir().unit_vector();
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new_singleton(1.0) + t * Color::new(0.5, 0.7, 1.0)
}
fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
    const COLOR_SIZE: u64 = 256;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new_dfl();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2. - Vec3::new(0.0, 0.0, focal_length);

    // Render

    println!("P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, COLOR_SIZE - 1);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / (IMAGE_WIDTH - 1) as f64;
            let v = (j as f64) / (IMAGE_HEIGHT - 1) as f64;
            let r: Ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r);
            // let pixel_color: Color = Color::new(
            //     i as f64 / (IMAGE_WIDTH - 1) as f64,
            //     j as f64 / (IMAGE_HEIGHT - 1) as f64,
            //     0.25,
            // );
            write_color(pixel_color);
        }
    }
    eprintln!("\nDone.");
}
