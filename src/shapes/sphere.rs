use super::{
    hit_record::{is_front_face, HitRecord},
    traits::Hit,
};
use crate::{math::vec3::Vec3, util::ray::Ray};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;

        let a = ray.direction().length().powi(2);
        let half_b = oc.dot(&ray.direction());
        let c = oc.length().powi(2) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find nearest root in range from t_min to t_max
        let discriminant_sqrt = discriminant.sqrt();

        let root = (-half_b - discriminant_sqrt) / a;
        if t_min <= root && root <= t_max {
            return Some(self.root_to_hit_record(root, ray));
        }

        let root = (-half_b + discriminant_sqrt) / a;
        if t_min <= root && root <= t_max {
            return Some(self.root_to_hit_record(root, ray));
        }

        None
    }
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    fn root_to_hit_record(&self, root: f32, ray: &Ray) -> HitRecord {
        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;
        let is_front_face = is_front_face(&ray, &normal);

        HitRecord {
            point,
            normal: match is_front_face {
                true => normal,
                false => -normal,
            },
            t: root,
            front_face: is_front_face,
        }
    }
}
