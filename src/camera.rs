use core::f64;

use crate::{
    ray::Ray,
    utils::degrees_to_radians,
    vec3::{Point3, Vec3},
};

#[derive()]
pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h: f64 = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        // let focal_length = 1.0;
        // let origin = Point3::new_dfl();
        // let hori = Vec3::new(viewport_width, 0.0, 0.0);
        // let vert = Vec3::new(0.0, viewport_height, 0.0);
        // let llcn = origin - hori / 2.0 - vert / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        let origin = lookfrom;
        let horizontal = viewport_width * u * focus_dist;
        let vertical = viewport_height * v * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;
        let lens_radius = aperture / 2.0;
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
        }
    }

    // pub fn new(
    //     origin: Point3,
    //     lower_left_corner: Point3,
    //     horizontal: Vec3,
    //     vertical: Vec3,
    // ) -> Self {
    //     Self {
    //         origin,
    //         lower_left_corner,
    //         horizontal,
    //         vertical,
    //     }
    // }
    // pub fn new_dfl() -> Self {
    //     let aspect_ratio = 16.0 / 9.0;
    //     let viewport_height = 2.0;
    //     let viewport_width = aspect_ratio * viewport_height;
    //     let focal_length = 1.0;
    //     let origin = Point3::new_dfl();
    //     let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    //     let vertical = Vec3::new(0.0, viewport_height, 0.0);
    //     let lower_left_corner =
    //         origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    //     Self::new(origin, lower_left_corner, horizontal, vertical)
    // }

    /// Get a reference to the camera's origin.
    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    /// Get a reference to the camera's lower left corner.
    pub fn lower_left_corner(&self) -> &Point3 {
        &self.lower_left_corner
    }

    /// Get a reference to the camera's horizontal.
    pub fn horizontal(&self) -> &Vec3 {
        &self.horizontal
    }

    /// Get a reference to the camera's vertical.
    pub fn vertical(&self) -> &Vec3 {
        &self.vertical
    }

    // pub fn get_ray(&self, u: f64, v: f64) -> Ray {
    //     Ray::new(
    //         *self.origin(),
    //         *self.lower_left_corner() + u * *self.horizontal() + v * *self.vertical()
    //             - *self.origin(),
    //     )
    // }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = *self.lens_radius() * Vec3::random_in_unit_disk();
        let offset = *self.u() * rd.x() + *self.v() * rd.y();
        Ray::new(
            *self.origin() + offset,
            *self.lower_left_corner() + u * *self.horizontal() + v * *self.vertical()
                - *self.origin()
                - offset,
        )
    }

    /// Get a reference to the camera's lens radius.
    pub fn lens_radius(&self) -> &f64 {
        &self.lens_radius
    }

    /// Get a reference to the camera's u.
    pub fn u(&self) -> &Vec3 {
        &self.u
    }

    /// Get a reference to the camera's v.
    pub fn v(&self) -> &Vec3 {
        &self.v
    }
}
