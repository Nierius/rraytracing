use crate::{shapes::hit_record::HitRecord, util::ray::Ray};

use super::scatter_record::ScatterRecord;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord) -> Option<ScatterRecord>;
}
