use super::*;
pub struct CpuFrameBuffer {
    id: Uuid,
    pub(crate) color_image: egui::ColorImage,
}

impl CpuFrameBuffer {
    pub fn new(size: UVec2) -> Self {
        Self {
            id: Uuid::new_v4(),
            color_image: egui::ColorImage::new((size).as_array(), egui::Color32::BLACK),
        }
    }
}

impl Texture for CpuFrameBuffer {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn size(&self) -> UVec2 {
        self.color_image.size.as_uvec2()
    }
}

impl FrameBuffer for CpuFrameBuffer {
    fn as_egui_texture_id(&self, ctx: &egui::Context) -> egui::TextureId {
        ctx.load_texture(
            self.id.to_string(),
            egui::ImageData::Color(Arc::new(self.color_image.clone())),
            egui::TextureOptions {
                magnification: egui::TextureFilter::Nearest,
                minification: egui::TextureFilter::Nearest,
            },
        )
        .id()
    }
}

impl RenderTarget for CpuFrameBuffer {
    fn image_scale(&self) -> f32 {
        1.0
    }
}
