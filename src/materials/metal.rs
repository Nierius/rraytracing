use crate::{
    math::vec3::Vec3,
    shapes::hit_record::HitRecord,
    util::{color::Color, ray::Ray},
};

use super::{material::Material, scatter_record::ScatterRecord};

pub struct Metal {
    pub albedo: Color,
    pub fuzziness: f32,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(ray_in.unit_direction(), hit_rec.normal);

        let scattered_ray = Ray::new(
            hit_rec.point,
            reflected + Vec3::random_in_unit_sphere() * self.fuzziness,
        );
        let attenuation = self.albedo;

        if scattered_ray.direction().dot(&hit_rec.normal) <= 0.0 {
            return None;
        }

        Some(ScatterRecord {
            scattered_ray,
            attenuation,
        })
    }
}

fn reflect(vec: Vec3, normal: Vec3) -> Vec3 {
    return (vec - 2.0) * vec.dot(&normal) * normal;
}
