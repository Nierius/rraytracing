use crate::math::vec3::Vec3;

pub fn refract(unit_vec: Vec3, normal: &Vec3, refraction_ratio: f32) -> Vec3 {
    let cos_theta = f32::min((-unit_vec).dot(normal), 1.0);

    let ray_out_perpendicular = (unit_vec + normal * cos_theta) * refraction_ratio;
    let ray_out_parallel = normal * -((1.0 - ray_out_perpendicular.length_squared()).abs()).sqrt();

    ray_out_perpendicular + ray_out_parallel
}

pub fn reflect(vec: Vec3, normal: &Vec3) -> Vec3 {
    return (vec - 2.0) * vec.dot(normal) * normal;
}
