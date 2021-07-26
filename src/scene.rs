// use rand::Rng;

use rand::Rng;

use crate::hittable::HittableObject;
use crate::material::MaterialType;
use crate::utils::{random, random_range};
use crate::vec3::{Color, Point};

#[allow(dead_code)]
pub fn img_11() -> HittableObject {
    let mut world = Vec::<HittableObject>::new();
    // let ground_mat = Box::new(MaterialType::Lambertian(Color::new(0.8, 0.8, 0.)));
    let mat_ground = MaterialType::Lambertian(Color::new(0.8, 0.8, 0.));
    let ground = HittableObject::Sphere(Point::new(0., -100.5, -1.), 100., mat_ground);
    world.push(ground);
    let mat_center = MaterialType::Lambertian(Color::new(0.7, 0.3, 0.3));
    let center = HittableObject::Sphere(Point::new(0., 0., -1.), 0.5, mat_center);
    world.push(center);
    let mat_left = MaterialType::Metal(Color::new_singleton(0.8), 0.);
    let left = HittableObject::Sphere(Point::new(-1., 0., -1.), 0.5, mat_left);
    world.push(left);
    let mat_right = MaterialType::Metal(Color::new(0.8, 0.6, 0.2), 0.);
    let right = HittableObject::Sphere(Point::new(1., 0., -1.), 0.5, mat_right);
    world.push(right);
    HittableObject::HittableList(world)
}

pub fn random_scene<R: Rng + ?Sized>(rng: &mut R) -> HittableObject {
    let mut world = Vec::<HittableObject>::new();
    let ground_material = MaterialType::Lambertian(Color::new(0.5, 0.5, 0.5));
    world.push(HittableObject::Sphere(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random(rng);
            let center = Point::new(
                a as f32 + 0.9 * random(rng),
                0.2,
                b as f32 + 0.9 * random(rng),
            );
            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material = match choose_mat {
                    x if x < 0.8 => {
                        MaterialType::Lambertian(Color::random_vec3(rng) * Color::random_vec3(rng))
                    }
                    x if x < 0.95 => MaterialType::Metal(
                        Color::random_vec3_range(0.5, 1.0, rng),
                        random_range(0.0, 0.5, rng),
                    ),
                    _ => (MaterialType::Dielectric(1.5)),
                };
                world.push(HittableObject::Sphere(center, 0.2, sphere_material));
            }
        }
    }
    let material1 = MaterialType::Dielectric(1.5);
    world.push(HittableObject::Sphere(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    ));
    let material2 = MaterialType::Lambertian(Color::new(0.4, 0.2, 0.1));
    world.push(HittableObject::Sphere(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    ));
    let material3 = MaterialType::Metal(Color::new(0.7, 0.6, 0.5), 0.0);
    world.push(HittableObject::Sphere(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    ));
    HittableObject::HittableList(world)
}
#[allow(dead_code)]
pub fn debug_scene() -> HittableObject {
    let mut world = Vec::<HittableObject>::new();
    let material = MaterialType::Dielectric(1.5);
    world.push(HittableObject::Sphere(Point::new_dfl(), 1., material));
    HittableObject::HittableList(world)
}
