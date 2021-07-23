use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive()]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        origin: Point3,
        lower_left_corner: Point3,
        horizontal: Vec3,
        vertical: Vec3,
    ) -> Self {
        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
    pub fn new_dfl() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let origin = Point3::new_dfl();
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
        Self::new(origin, lower_left_corner, horizontal, vertical)
    }

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

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            *self.origin(),
            *self.lower_left_corner() + u * *self.horizontal() + v * *self.vertical()
                - *self.origin(),
        )
    }
}
