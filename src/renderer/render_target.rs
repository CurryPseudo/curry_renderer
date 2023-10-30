pub trait RenderTarget {
    fn as_egui_color_image_mut(&mut self) -> &mut egui::ColorImage;
    fn image_scale(&self) -> f32;
}
