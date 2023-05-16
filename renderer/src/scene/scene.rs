use std::{f32::consts::PI, rc::Rc};

use super::camera::Camera;
use crate::{
    materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    math::vec3::Vec3,
    shapes::{hit_collection::HitCollection, sphere::Sphere},
    util::{color::Color, point::Point},
};

pub struct Scene {
    pub camera: Camera,
    pub world: HitCollection,
}

impl Default for Scene {
    fn default() -> Self {
        // Materials
        let mat_ground = Rc::new(Lambertian {
            albedo: Color::new([0.8, 0.8, 0.0]),
        });
        let mat_center = Rc::new(Lambertian {
            albedo: Color::new([0.7, 0.3, 0.3]),
        });
        let mat_left = Rc::new(Dielectric {
            refraction_index: 1.5,
        });
        let mat_right = Rc::new(Metal {
            albedo: Color::new([0.8, 0.6, 0.2]),
            fuzziness: 1.0,
        });

        // World
        let mut world = HitCollection::default();
        world.add(Box::new(Sphere::new(
            Point::new([0.0, -100.5, -1.0]),
            100.0,
            mat_ground,
        )));
        world.add(Box::new(Sphere::new(
            Point::new([-1.0, 0.0, -1.0]),
            0.5,
            mat_left,
        )));
        world.add(Box::new(Sphere::new(
            Point::new([0.0, 0.0, -1.0]),
            0.5,
            mat_center,
        )));
        world.add(Box::new(Sphere::new(
            Point::new([1.0, 0.0, -1.0]),
            0.5,
            mat_right,
        )));

        // Camera
        let look_from = Point::new([3.0, 3.0, 2.0]);
        let look_at = Point::new([0.0, 0.0, -1.0]);
        let view_up = Vec3::new([0.0, 1.0, 0.0]);
        let dist_to_focus = (look_from - look_at).length();
        let aperture = 2.0;
        let camera = Camera::new(
            look_from,
            look_at,
            view_up,
            PI / 180.0 * 20.0,
            16.0 / 9.0,
            aperture,
            dist_to_focus,
        );

        Scene { camera, world }
    }
}
