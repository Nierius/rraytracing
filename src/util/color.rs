use crate::math::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(color: Color, samples_per_pixel: f32) {
    println!(
        "{} {} {}",
        multivalue_to_unnormalized(color.x(), samples_per_pixel),
        multivalue_to_unnormalized(color.y(), samples_per_pixel),
        multivalue_to_unnormalized(color.z(), samples_per_pixel),
    )
}

// TODO BETTER NAMING
fn multivalue_to_unnormalized(val: f32, samples_per_pixel: f32) -> i16 {
    // sqrt for Gamma correction (2.0)
    let divided_val = (val / samples_per_pixel).sqrt();
    (256.0 * clamp(divided_val, 0.0, 0.999)) as i16
}

fn clamp(val: f32, min: f32, max: f32) -> f32 {
    if val < min {
        return min;
    }

    if val > max {
        return max;
    }

    val
}

// TESTS
