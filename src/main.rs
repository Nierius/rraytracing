mod math;
mod scene;
mod shapes;
mod util;
use std::io::{self, Write};

use rand::Rng;
use shapes::traits::Hit;
use util::{
    color::{write_color, Color},
    ray::Ray,
};

use crate::{
    scene::camera::Camera,
    shapes::{hit_collection::HitCollection, sphere::Sphere},
    util::point::Point,
};

fn main() {
    write_img();
}

fn ray_color(ray: &Ray, world: &HitCollection) -> Color {
    let hit_res = world.hit(ray, 0.0, f32::MAX);
    match hit_res {
        Some(hit) => return (hit.normal + Color::new([1.0, 1.0, 1.0])) * 0.5,
        None => (),
    }

    let unit_direction = ray.unit_direction();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new([1.0, 1.0, 1.0]) * (1.0 - t) + Color::new([0.5, 0.7, 1.0]) * t
}

fn write_img() {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i16 = 400;
    const IMAGE_HEIGHT: i16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i16;
    const SAMPLES_PER_PIXEL: i16 = 100; // This anti-aliasing makes it very very slow

    // World
    let mut world = HitCollection::default();
    world.add(Box::new(Sphere::new(Point::new([0.0, 0.0, -1.0]), 0.5)));
    world.add(Box::new(Sphere::new(
        Point::new([0.0, -100.5, -1.0]),
        100.0,
    )));

    // Camera
    let camera = Camera::new();

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");
    for y in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {y}");
        io::stderr().flush().unwrap_or_default();
        for x in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new([0.0, 0.0, 0.0]);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (x as f32 + rand_f32()) / (IMAGE_WIDTH - 1) as f32;
                let v = (y as f32 + rand_f32()) / (IMAGE_HEIGHT - 1) as f32;
                let ray = camera.get_ray(u, v);

                pixel_color = pixel_color + ray_color(&ray, &world);
            }

            write_color(pixel_color, SAMPLES_PER_PIXEL.into());
        }
    }
    eprintln!("All done.");
}

fn rand_f32() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}