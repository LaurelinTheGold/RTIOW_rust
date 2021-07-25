use std::{f32::INFINITY, iter::Sum, u8, usize};

use rand::Rng;

use crate::{
    camera::Camera,
    hittable::{HitRecord, Hittable, HittableObject},
    material::Material,
    ray::Ray,
    utils::random,
    vec3::{Color, Vec3},
};

pub fn ray_color<'b, R: Rng + ?Sized>(
    r: Ray,
    world: &'b HittableObject,
    depth: usize,
    rng: &mut R,
) -> Color {
    if depth <= 0 {
        Color::new_dfl()
    } else {
        match world.hit(r, 0.001, INFINITY) {
            Some(rec) => match rec.mat_ptr().scatter(r, &rec, rng) {
                Some(scatter_bundle) => {
                    scatter_bundle.albedo() * ray_color(scatter_bundle.ray(), world, depth - 1, rng)
                }
                None => Color::new_dfl(),
            },
            None => {
                let unit_direction = r.dir().unit_vector();
                let t = 0.5 * (unit_direction.y() + 1.0);
                (1.0 - t) * Color::new_singleton(1.0) + t * Color::new(0.5, 0.7, 1.0)
            }
        }
        // let mut rec = HitRecord::new_dfl();
        // if world.hit(r, 0.001, INFINITY, &mut rec) {
        //     let mut scattered: Ray = Ray::new_dfl();
        //     let mut attenuation: Color = Color::new_dfl();
        //     if rec
        //         .mat_ptr()
        //         .scatter(r, &rec, &mut attenuation, &mut scattered)
        //     {
        //         return attenuation * ray_color(&scattered, world, depth - 1);
        //     }
        //     return Color::new_dfl();
        // }
    }
}

/// Note colors are 3 u8 in u32 with 8 msb set to 0
fn render_a_row<R: Rng + ?Sized>(
    world: &HittableObject,
    max_depth: usize,
    width: usize,
    height: usize,
    curr_row: usize,
    samples_per_pixel: usize,
    cam: &Camera,
    rng: &mut R,
) -> Vec<u32> {
    (0..width)
        .into_iter()
        .map(|curr_col| {
            (0..samples_per_pixel)
                .into_iter()
                .map(|_x| {
                    get_ray_color(
                        world, max_depth, width, height, curr_row, curr_col, &cam, rng,
                    )
                })
                .fold(Vec3::new_dfl(), |boi, food| boi + food)
        })
        .map(|color| color_to_bitboi(color, samples_per_pixel))
        .collect::<Vec<u32>>()
}

fn color_to_bitboi(c: Color, samples_per_pixel: usize) -> u32 {
    let scale = 1.0 / samples_per_pixel as f32;
    let color_scale: f32 = 255.999;
    let zeroish: f32 = 0.0;
    let oneish: f32 = 0.999;
    let map_to_u8 = |x: f32| ((x * scale).sqrt().clamp(zeroish, oneish) * color_scale) as u8;
    0 | (map_to_u8(c.x()) as u32) << 16 | (map_to_u8(c.y()) as u32) << 8 | (map_to_u8(c.z()) as u32)
}

fn get_ray_color<R: Rng + ?Sized>(
    world: &HittableObject,
    max_depth: usize,
    width: usize,
    height: usize,
    curr_row: usize,
    curr_col: usize,
    cam: &Camera,
    rng: &mut R,
) -> Color {
    let u = (curr_col as f32 + random(rng)) / (width - 1) as f32;
    let v = (curr_row as f32 + random(rng)) / (height - 1) as f32;
    let r = cam.get_ray(u, v, rng);
    ray_color(r, world, max_depth, rng)
}

pub fn render_scene<R: Rng + ?Sized>(
    world: &HittableObject,
    max_depth: usize,
    width: usize,
    height: usize,
    samples_per_pixel: usize,
    cam: Camera,
    rng: &mut R,
) -> Vec<Vec<u32>> {
    (0..height)
        .rev()
        .into_iter()
        .map(|row| {
            eprint!("\rlines remaining: {}", row);
            render_a_row(
                world,
                max_depth,
                width,
                height,
                row,
                samples_per_pixel,
                &cam,
                rng,
            )
        })
        .collect::<Vec<_>>()
}

// pub fn render_scene() {
//     // Render
//     println!("P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, COLOR_SIZE - 1);

//     for j in (0..IMAGE_HEIGHT).rev() {
//         eprint!("\rScanlines remaining: {} ", j);
//         io::stderr().flush().unwrap();
//         for i in 0..IMAGE_WIDTH {
//             let mut pixel_color = Color::new_dfl();
//             for _s in 0..SAMPLES_PER_PIXEL {
//                 let u = (i as f32 + random_double()) / (IMAGE_WIDTH - 1) as f32;
//                 let v = (j as f32 + random_double()) / (IMAGE_HEIGHT - 1) as f32;
//                 let r = cam.get_ray(u, v);
//                 pixel_color += ray_color(&r, &world, MAX_DEPTH);
//             }
//             write_color(pixel_color, SAMPLES_PER_PIXEL);
//         }
//     }
//     eprintln!("\nDone.");
// }
