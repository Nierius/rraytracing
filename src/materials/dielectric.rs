use crate::{
    math::{random::rand_f32, vec3::Vec3},
    shapes::hit_record::HitRecord,
    util::{color::Color, ray::Ray},
};

use super::{
    interactions::{reflect, refract},
    material::Material,
    scatter_record::ScatterRecord,
};

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

        let unit_direction = Vec3::unit_vector(&ray_in.direction());
        let cos_theta = f32::min((-unit_direction).dot(&hit_rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let can_refract = refraction_ratio * sin_theta <= 1.0;

        let direction = if !can_refract || reflectance(cos_theta, refraction_ratio) > rand_f32() {
            refract(unit_direction, &hit_rec.normal, refraction_ratio)
        } else {
            reflect(unit_direction, &hit_rec.normal)
        };

        Some(ScatterRecord {
            scattered_ray: Ray::new(hit_rec.point, direction),
            attenuation,
        })
    }
}

/**
 * Schlick's approximation
 */
fn reflectance(cos: f32, refraction_ratio: f32) -> f32 {
    let r = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
    let r_pow = r * r;

    r_pow + (1.0 - r_pow) * (1.0 - cos).powi(5)
}
