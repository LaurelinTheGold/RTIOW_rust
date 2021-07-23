use std::{
    f64::{consts::PI, INFINITY},
    io::{self, Write},
    rc::Rc,
};

use hit::{HitRecord, Hittable};

use material::Material;
use ray::Ray;
use utils::random_double_range;
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

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new_dfl();
    }
    let mut rec = HitRecord::new_dfl();
    if world.hit(r, 0.001, INFINITY, &mut rec) {
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

fn random_scene() -> HittableList {
    let mut world = HittableList::new_dfl();
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material> = match choose_mat {
                    x if x < 0.8 => {
                        Rc::new(Lambertian::new(Color::random_vec3() * Color::random_vec3()))
                    }
                    x if x < 0.95 => Rc::new(Metal::new(
                        Color::random_vec3_range(0.5, 1.0),
                        random_double_range(0.0, 0.5),
                    )),
                    _ => Rc::new(Dielectric::new(1.5)),
                };
                world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }
    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    world
}
fn main() {
    // // Image
    // const COLOR_SIZE: u64 = 256;

    // const ASPECT_RATIO: f64 = 16.0 / 9.0;
    // const IMAGE_WIDTH: u64 = 400;
    // const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
    // const SAMPLES_PER_PIXEL: usize = 100;
    // const MAX_DEPTH: i32 = 50;

    // // World
    // let mut world = HittableList::new_dfl();
    // let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    // let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    // let material_left = Rc::new(Dielectric::new(1.5));
    // let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));
    // let tmp = material_left.clone();

    // world.add(Rc::new(Sphere::new(
    //     Point3::new(0.0, -100.5, -1.0),
    //     100.0,
    //     material_ground,
    // )));
    // world.add(Rc::new(Sphere::new(
    //     Point3::new(0.0, 0.0, -1.0),
    //     0.5,
    //     material_center,
    // )));
    // world.add(Rc::new(Sphere::new(
    //     Point3::new(-1.0, 0.0, -1.0),
    //     0.5,
    //     material_left,
    // )));
    // world.add(Rc::new(Sphere::new(
    //     Point3::new(-1.0, 0.0, -1.0),
    //     -0.4,
    //     tmp,
    // )));
    // world.add(Rc::new(Sphere::new(
    //     Point3::new(1.0, 0.0, -1.0),
    //     0.5,
    //     material_right,
    // )));

    // Camera
    // let cam = Camera::new(
    //     Point3::new(-2.0, 2.0, 1.0),
    //     Point3::new(0.0, 0.0, -1.0),
    //     Vec3::new(0.0, 1.0, 0.0),
    //     20.0,
    //     ASPECT_RATIO,
    // );
    // let lookfrom = Point3::new(3.0, 3.0, 2.0);
    // let lookat = Point3::new(0.0, 0.0, -1.0);
    // let vup = Vec3::new(0.0, 1.0, 0.0);
    // let focus_dist = (lookfrom - lookat).length();
    // let cam = Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, 2.0, focus_dist);

    // // World
    // let r = (PI / 4.0).cos();
    // let mut world = HittableList::new_dfl();
    // let material_left = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    // let material_right = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));
    // world.add(Rc::new(Sphere::new(
    //     Point3::new(-r, 0.0, -1.0),
    //     r,
    //     material_left,
    // )));
    // world.add(Rc::new(Sphere::new(
    //     Point3::new(r, 0.0, -1.0),
    //     r,
    //     material_right,
    // )));
    // // Camera
    // let cam = Camera::new(90.0, ASPECT_RATIO);

    // Image
    const COLOR_SIZE: u64 = 256;
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u64 = 1200;
    const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: usize = 500;
    const MAX_DEPTH: i32 = 50;
    // World
    let world = random_scene();
    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new_dfl();
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

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
