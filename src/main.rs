use crate::color::write_color;
use crate::ray::Ray;
use crate::render::ray_color;
use crate::vec3::{Point, Vec3};
use crate::{camera::Camera, color::print_output, render::render_scene};
use rand::prelude::StdRng;
use rand::{thread_rng, SeedableRng};
mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod render;
mod scene;
mod utils;
mod vec3;

fn main() {
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);
    // let mut rng = thread_rng();
    // rand::SeedableRng::Seed::rand::SeedableRng::seed_from_u64(0);
    // thread_rng();

    // Image
    // const COLOR_SIZE: usize = 256;
    // const ASPECT_RATIO: f32 = 16. / 9.;
    // const IMAGE_WIDTH: usize = 400;
    // const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;
    // const SAMPLES_PER_PIXEL: usize = 100;
    // const MAX_DEPTH: usize = 50;
    // let world = scene::img_11();
    // let cam = Camera::new_dfl();

    const COLOR_SIZE: usize = 256;
    const ASPECT_RATIO: f32 = 16. / 10.;
    const IMAGE_WIDTH: usize = 1920;
    // const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 500;
    // const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;
    let world = scene::random_scene(&mut rng);
    let cam = Camera::new_random(ASPECT_RATIO);

    let image = render_scene(
        &world,
        MAX_DEPTH,
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        SAMPLES_PER_PIXEL,
        cam,
        &mut rng,
    );
    print_output(image, IMAGE_WIDTH, IMAGE_HEIGHT, COLOR_SIZE);

    // //? For debug
    // const SAMPLES_PER_PIXEL: usize = 100;
    // let world = scene::debug_scene();
    // write_color(
    //     ray_color(
    //         Ray::new(Point::new(0., 0., 2.), Vec3::new(0., 0., -1.)),
    //         &world,
    //         4,
    //         &mut rng,
    //     ),
    //     SAMPLES_PER_PIXEL,
    // );
}
