use crate::util::{color::Color, ray::Ray};

pub struct ScatterRecord {
    pub scattered_ray: Ray,
    pub attenuation: Color,
}
