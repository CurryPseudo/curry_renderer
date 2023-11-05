use crate::*;
pub trait FrameBuffer: Texture {
    fn as_egui_texture_id(&self, ctx: &egui::Context) -> egui::TextureId;
}
