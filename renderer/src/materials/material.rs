use crate::{shapes::hit_record::HitRecord, util::ray::Ray};

use super::{
    dielectric::Dielectric, lambertian::Lambertian, metal::Metal, scatter_record::ScatterRecord,
};

#[derive(Clone)]
pub enum MaterialType {
    Dielectric(Dielectric),
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material for MaterialType {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<ScatterRecord> {
        match self {
            MaterialType::Dielectric(dielectric) => dielectric.scatter(ray_in, hit_rec),
            MaterialType::Metal(metal) => metal.scatter(ray_in, hit_rec),
            MaterialType::Lambertian(lambertian) => lambertian.scatter(ray_in, hit_rec),
        }
    }
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<ScatterRecord>;
}
