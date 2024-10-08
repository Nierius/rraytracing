use crate::{
    materials::material::MaterialType,
    math::vec3::Vec3,
    util::{point::Point, ray::Ray},
};

pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: MaterialType,
}

pub fn is_front_face(ray: &Ray, outward_normal: &Vec3) -> bool {
    ray.direction().dot(outward_normal) < 0.0
}
