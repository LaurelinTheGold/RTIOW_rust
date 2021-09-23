use rand::Rng;

use crate::{
    ray::Ray,
    utils::degrees_to_radians,
    vec3::{Point, Vec3},
};

pub struct Camera {
    origin: Point,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Point,
        lookat: Point,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h: f32 = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

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
    pub fn new_dfl(aspect_ratio: f32) -> Self {
        let viewport_height = 2.;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.;
        let origin = Point::new_dfl();
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);
        let w = Vec3::new(0., 0., -1.);
        let u = Vec3::new(-1., 0., 0.);
        let v = w.cross(u);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius: (0.0),
        }
    }
    pub fn new_debug(aspect_ratio: f32) -> Self {
        let viewport_height = 2.;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.;
        let origin = Point::new(0., 0., 2.);
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);
        let w = Vec3::new(0., 0., -1.);
        let u = Vec3::new(-1., 0., 0.);
        let v = w.cross(u);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius: (0.0),
        }
    }

    pub fn new_random(aspect_ratio: f32) -> Self {
        Self::new(
            Point::new(13., 2., 3.),
            Point::new_dfl(),
            Vec3::new(0., 1., 0.),
            20.,
            aspect_ratio,
            0.1,
            10.,
        )
    }

    /// Get a reference to the camera's origin.
    pub fn origin(&self) -> &Point {
        &self.origin
    }

    /// Get a reference to the camera's lower left corner.
    pub fn lower_left_corner(&self) -> &Point {
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

    pub fn get_ray<R: Rng + ?Sized>(&self, u: f32, v: f32, rng: &mut R) -> Ray {
        let rd = *self.lens_radius() * Vec3::random_in_unit_disk(rng);
        let offset = *self.u() * rd.x() + *self.v() * rd.y();
        Ray::new(
            *self.origin() + offset,
            *self.lower_left_corner() + u * *self.horizontal() + v * *self.vertical()
                - *self.origin()
                - offset,
        )
    }

    /// Get a reference to the camera's lens radius.
    pub fn lens_radius(&self) -> &f32 {
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
