use crate::{
    math::vec3::Vec3,
    util::{point::Point, ray::Ray},
};

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Point,
        look_to: Point,
        view_up: Vec3,
        vfov_rad: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let h = (vfov_rad / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let w = Vec3::unit_vector(&(look_from - look_to));
        let u = Vec3::unit_vector(&view_up.cross(&w));
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset,
        )
    }
}
