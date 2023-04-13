pub trait Render {
    fn render(
        &self,
        scene: crate::data::Scene,
        frame_width: i32,
        frame_height: i32,
    ) -> crate::data::Frame;
}
