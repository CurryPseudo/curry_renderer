use crate::*;
pub trait FrameBuffer {
    fn size(&self) -> UVec2;
    fn resize(&mut self, new_size: UVec2);
    fn as_render_target_mut(&mut self) -> &mut dyn RenderTarget;
    fn as_egui_texture_id(&self, ctx: &egui::Context) -> egui::TextureId;
}
