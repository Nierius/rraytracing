use std::{f32::consts::PI, rc::Rc};

use shared::traits::Render;

use crate::{
    materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    math::{random::rand_f32, vec3::Vec3},
    scene::camera::Camera,
    shapes::{hit_collection::HitCollection, sphere::Sphere, traits::Hit},
    util::{color::Color, point::Point, ray::Ray},
};

fn ray_color_material(ray: &Ray, world: &HitCollection, depth: i16) -> Color {
    // Recursion protection
    if depth <= 0 {
        return Color::default();
    }

    let hit_res = world.hit(ray, 0.001, f32::MAX);
    match hit_res {
        Some(hit) => {
            let scatter_res = hit.material.scatter(ray, &hit);
            match scatter_res {
                Some(scatter) => {
                    return scatter.attenuation
                        * ray_color_material(&scatter.scattered_ray, world, depth - 1)
                }
                None => return Color::default(),
            }
        }
        None => (),
    }

    let unit_direction = ray.unit_direction();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new([1.0, 1.0, 1.0]) * (1.0 - t) + Color::new([0.5, 0.7, 1.0]) * t
}

pub struct Renderer {}

impl Render for Renderer {
    fn render(
        &self,
        scene: shared::data::Scene,
        frame_width: i32,
        frame_height: i32,
    ) -> shared::data::Frame {
        todo!()
    }

    fn render_pixel(
        &self,
        scene: shared::data::Scene,
        x: i32,
        y: i32,
        frame_width: i32,
        frame_height: i32,
    ) -> shared::data::Pixel {
        let aspect_ratio: f32 = frame_width as f32 / frame_height as f32;
        const SAMPLES_PER_PIXEL: i16 = 10; // This anti-aliasing makes it very very slow
        const MAX_DEPTH: i16 = 50;

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
            aspect_ratio,
            aperture,
            dist_to_focus,
        );

        let mut pixel_color = Color::default();
        for _ in 0..SAMPLES_PER_PIXEL {
            let u = (x as f32 + rand_f32()) / (frame_width - 1) as f32;
            let v = (y as f32 + rand_f32()) / (frame_height - 1) as f32;
            let ray = camera.get_ray(u, v);

            pixel_color = pixel_color + ray_color_material(&ray, &world, MAX_DEPTH);
        }

        shared::data::Pixel::new_f32(
            multivalue_to_unnormalized(pixel_color.x(), SAMPLES_PER_PIXEL.into()),
            multivalue_to_unnormalized(pixel_color.y(), SAMPLES_PER_PIXEL.into()),
            multivalue_to_unnormalized(pixel_color.z(), SAMPLES_PER_PIXEL.into()),
        )
    }
}

fn multivalue_to_unnormalized(val: f32, samples_per_pixel: f32) -> f32 {
    // sqrt for Gamma correction (2.0)
    let divided_val = (val / samples_per_pixel).sqrt();
    assert!(divided_val <= 1.0 && divided_val >= 0.0);
    divided_val
}
