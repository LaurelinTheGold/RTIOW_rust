use std::marker::{Send, Sync};
use std::{f32::INFINITY, u8, usize};

use rand::prelude::StdRng;
// use rand::prelude::StdRng;
use rand::{thread_rng, Rng, SeedableRng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::prelude::*;
use tailcall::tailcall;

use crate::{
    camera::Camera,
    hittable::{Hittable, HittableObject},
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
    // tested with single threaded, no target native cpu
    //? naive recursion, dump1, 11.206s
    // if depth <= 0 {
    //     Color::new_dfl()
    // } else {
    //     match world.hit(r, 0.001, INFINITY) {
    //         Some(rec) => match rec.mat_ptr().scatter(r, &rec, rng) {
    //             Some(scatter_bundle) => {
    //                 scatter_bundle.albedo() * ray_color(scatter_bundle.ray(), world, depth - 1, rng)
    //             }
    //             None => Color::new_dfl(),
    //         },
    //         None => sky_color(r),
    //     }
    // }
    //? Iterative, dump2, 11.124s
    let mut ret_color = Color::new_singleton(1.);
    let mut cur_ray = r;
    for tmp in (0..depth).rev() {
        if tmp == 0 {
            return Color::new_dfl();
        }
        if let Some(rec) = world.hit(cur_ray, 0.001, INFINITY) {
            if let Some(scatter_bundle) = rec.mat_ptr().scatter(cur_ray, &rec, rng) {
                ret_color = scatter_bundle.albedo() * ret_color;
                cur_ray = scatter_bundle.ray();
            } else {
                return Color::new_dfl();
            }
        } else {
            return ret_color * sky_color(cur_ray);
        }
    }
    ret_color
    //? (hopefully) tail call optimized recursion, dump3, 11.225s
    // #[tailcall]
    // fn ray_color_tail<'b, R: Rng + ?Sized>(
    //     r: Ray,
    //     world: &'b HittableObject,
    //     depth: usize,
    //     rng: &mut R,
    //     color_acc: Color,
    // ) -> Color {
    //     match depth <= 0 {
    //         true => Color::new_dfl(),
    //         false => match world.hit(r, 0.001, INFINITY) {
    //             Some(rec) => match rec.mat_ptr().scatter(r, &rec, rng) {
    //                 Some(scatter_bundle) => ray_color_tail(
    //                     scatter_bundle.ray(),
    //                     world,
    //                     depth - 1,
    //                     rng,
    //                     color_acc * scatter_bundle.albedo(),
    //                 ),
    //                 None => Color::new_dfl(),
    //             },
    //             None => color_acc * sky_color(r),
    //         },
    //     }
    // }
    // ray_color_tail(r, world, depth, rng, Color::new_singleton(1.))
}

fn sky_color(r: Ray) -> Color {
    let t = r.dir().unit_vector().y();
    (1.0 - t) * Color::new_singleton(1.0) + t * Color::new(0.5, 0.7, 1.0)
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
        // .into_par_iter()
        .into_iter()
        .map(|curr_col| -> Vec3 {
            // (0..samples_per_pixel)
            //     .into_par_iter()
            //     .map(|_x| -> Vec3 {
            //         let mut rng = thread_rng();
            //         get_ray_color(
            //             world, max_depth, width, height, curr_row, curr_col, &cam, &mut rng,
            //         )
            //     })
            //     .reduce(|| Vec3::new_dfl(), |boi, food| boi + food)
            // let mut rng = thread_rng();
            (0..samples_per_pixel)
                .into_iter()
                .map(|_x| -> Vec3 {
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
/// Do once for each in samplesperpixel
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

pub fn render_scene<R: Rng + ?Sized + Sync + Send>(
    world: &HittableObject,
    max_depth: usize,
    width: usize,
    height: usize,
    samples_per_pixel: usize,
    cam: Camera,
    _rng: &mut R,
    // rng: &mut R,
) -> Vec<Vec<u32>> {
    (0..height)
        .into_par_iter()
        // .into_iter()
        .rev()
        .map(|row| {
            let mut rng: StdRng = SeedableRng::seed_from_u64(row as u64);
            // eprint!("\rlines remaining: {}", row);
            render_a_row(
                world,
                max_depth,
                width,
                height,
                row,
                samples_per_pixel,
                &cam,
                &mut rng,
                // rng,
            )
        })
        .collect::<Vec<_>>()
}
