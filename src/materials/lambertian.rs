use crate::math::vec3::Vec3;
use crate::shapes::hit_record;
use crate::util::color::Color;
use crate::util::ray::Ray;

use super::material::Material;
use super::scatter_record::ScatterRecord;

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_rec: &hit_record::HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction = hit_rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_rec.normal;
        }

        let scattered_ray = Ray::new(hit_rec.point, scatter_direction);
        let attenuation = self.albedo;

        Some(ScatterRecord {
            scattered_ray,
            attenuation,
        })
    }
}
