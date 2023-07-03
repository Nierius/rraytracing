use super::vec3::Vec3;
use rand::{rngs::ThreadRng, Rng};

static mut RNG: Option<ThreadRng> = None;

unsafe fn get_rng() -> &'static mut ThreadRng {
    RNG.get_or_insert_with(ThreadRng::default)
}

pub fn rand_f32() -> f32 {
    let rng = unsafe { get_rng() };
    rng.gen()
}

/**
 * Inclusive clamping
 */
pub fn rand_f32_clamped(min: f32, max: f32) -> f32 {
    let rng = unsafe { get_rng() };
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
