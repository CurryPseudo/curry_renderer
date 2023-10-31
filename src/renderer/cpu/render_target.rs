use super::*;
pub struct CpuRenderTarget {
    pub(crate) color_image: egui::ColorImage,
    pub(crate) super_sampled_scale: u32,
}
impl CpuRenderTarget {
    pub fn new(size: UVec2, super_sampled_scale: u32) -> Self {
        Self {
            color_image: egui::ColorImage::new(
                (size * super_sampled_scale).as_array(),
                egui::Color32::BLACK,
            ),
            super_sampled_scale,
        }
    }
}
impl Texture for CpuRenderTarget {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn size(&self) -> UVec2 {
        self.color_image.size.as_uvec2() / self.super_sampled_scale
    }
}
impl RenderTarget for CpuRenderTarget {
    fn image_scale(&self) -> f32 {
        self.super_sampled_scale as f32
    }

    fn as_egui_color_image_mut(&mut self) -> &mut egui::ColorImage {
        &mut self.color_image
    }
}
