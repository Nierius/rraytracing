pub trait Render {
    fn render(
        &self,
        scene: crate::data::Scene,
        frame_width: i32,
        frame_height: i32,
    ) -> crate::data::Frame;

    fn render_pixel(
        &self,
        scene: crate::data::Scene,
        x: i32,
        y: i32,
        frame_width: i32,
        frame_height: i32,
    ) -> crate::data::Pixel;
}
