use crate::{
    math::vec3::Vec3,
    shapes::hit_record::HitRecord,
    util::{color::Color, ray::Ray},
};

use super::{material::Material, scatter_record::ScatterRecord};

pub struct Dielectric {
    pub refraction_index: f32,
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = Color::new([1.0, 1.0, 1.0]);
        let refraction_ratio = if hit_rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let refracted = refract(
            Vec3::unit_vector(&ray_in.direction()),
            &hit_rec.normal,
            refraction_ratio,
        );

        Some(ScatterRecord {
            scattered_ray: Ray::new(hit_rec.point, refracted),
            attenuation,
        })
    }
}

fn refract(unit_vec: Vec3, normal: &Vec3, refraction_ratio: f32) -> Vec3 {
    let cos_theta = f32::min((-unit_vec).dot(normal), 1.0);

    let ray_out_perpendicular = (unit_vec + normal * cos_theta) * refraction_ratio;
    let ray_out_parallel = normal * -((1.0 - ray_out_perpendicular.length_squared()).abs()).sqrt();

    ray_out_perpendicular + ray_out_parallel
}
