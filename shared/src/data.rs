/**
 * Output image/frame from renderer
 */
pub struct Frame {
    pub pixels: Vec<Pixel>,
    pub height: i32,
    pub width: i32,
}

pub struct Pixel {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}
