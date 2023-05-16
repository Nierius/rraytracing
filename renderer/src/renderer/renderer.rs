use shared::traits::Render;

use crate::{
    math::random::rand_f32,
    scene::scene::Scene,
    shapes::{hit_collection::HitCollection, traits::Hit},
    util::{
        color::{sampled_value_to_normalized, Color},
        ray::Ray,
    },
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

const MAX_RECURSION_DEPTH: i16 = 50;

pub struct Renderer {
    scene: Scene,
}

impl Render for Renderer {
    fn render(&self, frame_width: i32, frame_height: i32) -> shared::data::Frame {
        todo!()
    }

    fn render_pixel(
        &self,
        x: i32,
        y: i32,
        frame_width: i32,
        frame_height: i32,
    ) -> shared::data::Pixel {
        const SAMPLES_PER_PIXEL: i16 = 10; // This anti-aliasing makes it very very slow

        let mut pixel_color = Color::default();
        for _ in 0..SAMPLES_PER_PIXEL {
            let u = (x as f32 + rand_f32()) / (frame_width - 1) as f32;
            let v = (y as f32 + rand_f32()) / (frame_height - 1) as f32;
            let ray = self.scene.camera.get_ray(u, v);

            pixel_color =
                pixel_color + ray_color_material(&ray, &self.scene.world, MAX_RECURSION_DEPTH);
        }

        shared::data::Pixel {
            r: sampled_value_to_normalized(pixel_color.x(), SAMPLES_PER_PIXEL.into()).into(),
            g: sampled_value_to_normalized(pixel_color.y(), SAMPLES_PER_PIXEL.into()).into(),
            b: sampled_value_to_normalized(pixel_color.z(), SAMPLES_PER_PIXEL.into()).into(),
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
