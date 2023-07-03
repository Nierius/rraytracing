use crate::math::vec3::Vec3;

pub type Color = Vec3;

pub fn sampled_value_to_normalized(val: f32, samples_per_pixel: f32) -> f32 {
    // sqrt for Gamma correction (2.0)
    let divided_val = (val / samples_per_pixel).sqrt();
    assert!(divided_val <= 1.0 && divided_val >= 0.0);
    divided_val
}
