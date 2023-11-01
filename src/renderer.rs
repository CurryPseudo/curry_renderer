use crate::*;

mod render_target;
pub use render_target::*;
mod cpu;
pub use cpu::*;
mod frame_buffer;
pub use frame_buffer::*;
mod texture;
pub use texture::*;
pub trait Renderer {
    fn msaa_enable(&mut self) -> &mut bool;
    fn ssaa_enable(&mut self) -> &mut bool;
    fn frame_size(&self) -> UVec2;
    fn resize_frame(&mut self, new_size: UVec2);
    fn render_current_frame_if_ready(
        &mut self,
        f: Box<dyn Fn(&dyn RenderCommandList, &mut dyn FrameBuffer) + Send>,
    );
    fn last_frame_time(&self) -> std::time::Duration;
    fn present(&self, ctx: &egui::Context) -> egui::TextureId;
}

pub trait RenderCommandList {
    fn create_render_target(&self, size: UVec2) -> Box<dyn RenderTarget>;
    fn clear(&self, target: &mut dyn RenderTarget);
    fn draw_triangle(
        &self,
        triangle: &Triangle,
        colors: &[egui::Color32; 3],
        target: &mut dyn RenderTarget,
    );
    fn copy_render_target_to_frame_buffer(
        &self,
        source: &dyn RenderTarget,
        destination: &mut dyn FrameBuffer,
    );
}
