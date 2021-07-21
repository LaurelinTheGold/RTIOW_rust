use std::{
    f64::INFINITY,
    io::{self, Write},
    rc::Rc,
};

use hit::{HitRecord, Hittable};

use ray::Ray;
use vec3::Vec3;

use crate::{
    camera::Camera, color::write_color, hittable_list::HittableList, sphere::Sphere,
    utils::random_double, vec3::*,
};

mod camera;
mod color;
mod hit;
mod hittable_list;
mod ray;
mod sphere;
mod utils;
mod vec3;

// fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
//     use vec3::*;
//     let oc: Vec3 = r.orig() - *center;
//     let a = r.dir().length_squared();
//     let half_b = oc.dot(r.dir());
//     let c = oc.length_squared() - radius * radius;
//     let discriminant = half_b * half_b - a * c;
//     if discriminant < 0.0 {
//         -1.0
//     } else {
//         (-half_b - discriminant.sqrt()) / (a)
//     }
// }

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new_dfl();
    }
    let mut rec = HitRecord::new_dfl();
    if world.hit(r, 0.001, INFINITY, &mut rec) {
        // let target = *rec.p() + *rec.normal() + Vec3::random_in_unit_sphere();
        // let target = *rec.p() + *rec.normal() + Vec3::random_unit_vector();
        let target = *rec.p() + Vec3::random_in_hemisphere(rec.normal());
        return 0.5 * ray_color(&Ray::new(*rec.p(), target - *rec.p()), world, depth - 1);
        // return 0.5 * (*rec.normal() + Color::new_singleton(1.0));
    }
    let unit_direction: Vec3 = r.dir().unit_vector();
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new_singleton(1.0) + t * Color::new(0.5, 0.7, 1.0);
}
fn main() {
    // Image
    const COLOR_SIZE: u64 = 256;

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 400;
    const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = HittableList::new_dfl();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), -100.0)));

    // Camera
    let cam = Camera::new_dfl();

    // Render

    println!("P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, COLOR_SIZE - 1);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new_dfl();
            for s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("\nDone.");
}
