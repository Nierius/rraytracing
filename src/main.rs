mod materials;
mod math;
mod scene;
mod shapes;
mod util;
use std::{
    f32::consts::PI,
    io::{self, Write},
    rc::Rc,
};

use shapes::traits::Hit;
use util::{
    color::{write_color, Color},
    ray::Ray,
};

use crate::{
    materials::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    math::{random::rand_f32, vec3::Vec3},
    scene::camera::Camera,
    shapes::{hit_collection::HitCollection, sphere::Sphere},
    util::point::Point,
};

fn main() {
    write_img();
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

fn write_img() {
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i16 = 400;
    const IMAGE_HEIGHT: i16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i16;
    const SAMPLES_PER_PIXEL: i16 = 100; // This anti-aliasing makes it very very slow
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
    let camera = Camera::new(
        Point::new([-2.0, 2.0, 1.0]),
        Point::new([0.0, 0.0, -1.0]),
        Vec3::new([0.0, 1.0, 0.0]),
        PI / 2.0,
        ASPECT_RATIO,
    );

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");
    for y in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {y}");
        io::stderr().flush().unwrap_or_default();
        for x in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::default();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (x as f32 + rand_f32()) / (IMAGE_WIDTH - 1) as f32;
                let v = (y as f32 + rand_f32()) / (IMAGE_HEIGHT - 1) as f32;
                let ray = camera.get_ray(u, v);

                pixel_color = pixel_color + ray_color_material(&ray, &world, MAX_DEPTH);
            }

            write_color(pixel_color, SAMPLES_PER_PIXEL.into());
        }
    }
    eprintln!("All done.");
}
