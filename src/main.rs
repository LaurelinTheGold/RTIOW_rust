use std::{
    f64::INFINITY,
    io::{self, Write},
    rc::Rc,
};

use hit::{HitRecord, Hittable};

use ray::Ray;
use vec3::Vec3;

use crate::{
    camera::Camera,
    color::write_color,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    utils::random_double,
    vec3::*,
};

mod camera;
mod color;
mod hit;
mod hittable_list;
mod material;
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
        // // let target = *rec.p() + *rec.normal() + Vec3::random_in_unit_sphere();
        // // let target = *rec.p() + *rec.normal() + Vec3::random_unit_vector();
        // let target = *rec.p() + Vec3::random_in_hemisphere(rec.normal());
        // return 0.5 * ray_color(&Ray::new(*rec.p(), target - *rec.p()), world, depth - 1);
        // // return 0.5 * (*rec.normal() + Color::new_singleton(1.0));
        let mut scattered: Ray = Ray::new_dfl();
        let mut attenuation: Color = Color::new_dfl();
        if rec
            .mat_ptr()
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new_dfl();
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
    // world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    // world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), -100.0)));
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    // let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    // let material_left = Rc::new(Metal::new(Color::new_singleton(0.8), 0.3));
    let material_center = Rc::new(Dielectric::new(1.5));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let cam = Camera::new_dfl();

    // Render

    println!("P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, COLOR_SIZE - 1);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new_dfl();
            for _s in 0..SAMPLES_PER_PIXEL {
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

// ! For debugging use!
// fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
//     if depth <= 0 {
//         println!("\nBottom Out");
//         return Color::new_dfl();
//     }
//     println!("ray: {:?}, enter ray_color", r);
//     let mut rec = HitRecord::new_dfl();
//     if world.hit(r, 0.001, INFINITY, &mut rec) {
//         println!("hit: {:?}, world hit", rec);
//         let mut scattered: Ray = Ray::new_dfl();
//         let mut attenuation: Color = Color::new_dfl();
//         if rec
//             .mat_ptr()
//             .scatter(r, &rec, &mut attenuation, &mut scattered)
//         {
//             println!("new: {:?}, scattered ray", &scattered);
//             return attenuation * ray_color(&scattered, world, depth - 1);
//         }
//         println!("Hit no Scatter");
//         return Color::new_dfl();
//     }
//     let unit_direction: Vec3 = r.dir().unit_vector();
//     let t: f64 = 0.5 * (unit_direction.y() + 1.0);
//     println!("Background");
//     return (1.0 - t) * Color::new_singleton(1.0) + t * Color::new(0.5, 0.7, 1.0);
// }
// fn main() {
//     // Image
//     const SAMPLES_PER_PIXEL: usize = 25;
//     const MAX_DEPTH: i32 = 4;
//     // World
//     let mut world = HittableList::new_dfl();
//     let material_center = Rc::new(Dielectric::new(1.5));
//     world.add(Rc::new(Sphere::new(
//         Point3::new(0.0, 0.0, 0.0),
//         1.0,
//         material_center,
//     )));

//     // Eye
//     let eye_pos = Point3::new(0.0, 0.0, 2.0);
//     let eye_dir = Vec3::new(0.0, 0.0, -1.0);

//     // Render
//     let mut pixel_color = Color::new_dfl();
//     let r = Ray::new(eye_pos, eye_dir);
//     pixel_color += ray_color(&r, &world, MAX_DEPTH);

//     write_color(pixel_color, SAMPLES_PER_PIXEL);
// }
