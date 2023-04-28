/**
 * Object containing all (and only) the information necessary to render a scene
 */
pub struct Scene {
    //    pub camera: Camera,
    //    pub world: HitCollection,
}

/**
 * Output image/frame from renderer
 */
pub struct Frame {}

pub struct Pixel {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

// TODO: REMOVE
impl Pixel {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn new_f32(r: f32, g: f32, b: f32) -> Self {
        Self::new(r.into(), g.into(), b.into())
    }
}
