// TODO BETTER NAMING

use crate::util::ray::Ray;

use super::{hit_record::HitRecord, traits::Hit};

pub struct HitCollection {
    hittables: Vec<Box<dyn Hit>>,
}

impl Default for HitCollection {
    fn default() -> Self {
        HitCollection { hittables: vec![] }
    }
}

impl Hit for HitCollection {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_hit_record: Option<HitRecord> = None;

        for hittable in &self.hittables {
            let closest_distance = match &closest_hit_record {
                Some(record) => record.t,
                None => t_max,
            };

            let record = hittable.hit(ray, t_min, closest_distance);
            match record {
                Some(record) => {
                    closest_hit_record = Some(record);
                }
                None => continue,
            }
        }

        closest_hit_record
    }
}

impl HitCollection {
    pub fn add(&mut self, hittable: Box<dyn Hit>) {
        self.hittables.push(hittable)
    }
}
