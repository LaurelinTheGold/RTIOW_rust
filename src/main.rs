use crate::{camera::Camera, color::print_output, render::render_scene};
use rand::rngs::SmallRng;
use rand::SeedableRng;
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
    let mut rng = SmallRng::seed_from_u64(0);
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
    const ASPECT_RATIO: f32 = 3. / 2.;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;
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
}
