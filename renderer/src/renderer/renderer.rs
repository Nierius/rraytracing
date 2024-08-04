use shared::traits::Render;

use crate::{
    materials::material::Material,
    math::random::rand_f32,
    scene::scene::Scene,
    shapes::{hit_collection::HitCollection, traits::Hit},
    util::{
        color::{sampled_value_to_normalized, Color},
        ray::Ray,
    },
};

const MAX_RECURSION_DEPTH: i16 = 50;

pub struct Renderer {
    scene: Scene,
}

impl Render for Renderer {
    fn render(
        &self,
        frame_width: i32,
        frame_height: i32,
        samples_per_pixel: i16,
    ) -> shared::data::Frame {
        let mut pixels = vec![];
        for y in 0..frame_height {
            for x in 0..frame_width {
                pixels.push(self.render_pixel(x, y, frame_width, frame_height, samples_per_pixel))
            }
        }

        shared::data::Frame {
            pixels,
            height: frame_height,
            width: frame_width,
        }
    }

    fn render_pixel(
        &self,
        x: i32,
        y: i32,
        frame_width: i32,
        frame_height: i32,
        samples_per_pixel: i16,
    ) -> shared::data::Pixel {
        let mut pixel_color = Color::default();
        for _ in 0..samples_per_pixel {
            let u = (x as f32 + rand_f32()) / (frame_width - 1) as f32;
            let v = (y as f32 + rand_f32()) / (frame_height - 1) as f32;
            let ray = self.scene.camera.get_ray(u, v);

            pixel_color =
                pixel_color + ray_color_material(&ray, &self.scene.world, MAX_RECURSION_DEPTH);
        }

        shared::data::Pixel {
            r: sampled_value_to_normalized(pixel_color.x(), samples_per_pixel.into()).into(),
            g: sampled_value_to_normalized(pixel_color.y(), samples_per_pixel.into()).into(),
            b: sampled_value_to_normalized(pixel_color.z(), samples_per_pixel.into()).into(),
        }
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            scene: Default::default(),
        }
    }
}

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
