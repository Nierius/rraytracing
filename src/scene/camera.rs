use crate::{
    math::vec3::Vec3,
    util::{point::Point, ray::Ray},
};

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point::default();
        let horizontal = Vec3::new([viewport_width, 0.0, 0.0]);
        let vertical = Vec3::new([0.0, viewport_height, 0.0]);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new([0.0, 0.0, focal_length]);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
