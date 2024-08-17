use super::vec3::Vec3;
use rand::{thread_rng, Rng};

pub fn rand_f32() -> f32 {
    let mut rng = thread_rng();
    rng.gen()
}

/**
 * Inclusive clamping
 */
pub fn rand_f32_clamped(min: f32, max: f32) -> f32 {
    let mut rng = thread_rng();
    rng.gen_range(min..=max)
}

pub trait Random {
    fn random() -> Self;
    fn random_clamped(min: f32, max: f32) -> Self;
}

impl Random for Vec3 {
    fn random() -> Vec3 {
        Vec3 {
            e: ([rand_f32(), rand_f32(), rand_f32()]),
        }
    }

    fn random_clamped(min: f32, max: f32) -> Self {
        Vec3 {
            e: ([
                rand_f32_clamped(min, max),
                rand_f32_clamped(min, max),
                rand_f32_clamped(min, max),
            ]),
        }
    }
}
