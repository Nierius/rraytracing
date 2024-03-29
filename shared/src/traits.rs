pub trait Render {
    fn render(
        &self,
        frame_width: i32,
        frame_height: i32,
        samples_per_pixel: i16,
    ) -> crate::data::Frame;

    fn render_pixel(
        &self,
        x: i32,
        y: i32,
        frame_width: i32,
        frame_height: i32,
        samples_per_pixel: i16,
    ) -> crate::data::Pixel;
}
