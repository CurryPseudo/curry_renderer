use super::*;
impl RenderTarget for CpuFrameBuffer {
    fn as_egui_color_image_mut(&mut self) -> &mut egui::ColorImage {
        &mut self.color_image
    }

    fn image_scale(&self) -> f32 {
        self.super_sampled_scale as f32
    }
}
