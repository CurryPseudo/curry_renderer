use crate::*;

mod render_target;
pub use render_target::*;
mod cpu;
pub use cpu::*;
mod antialiasing;
pub use antialiasing::*;
mod frame_buffer;
pub use frame_buffer::*;
pub trait Renderer {
    fn clear(&self, target: &mut dyn RenderTarget);
    fn draw_triangle(&self, triangle: &Triangle, target: &mut dyn RenderTarget);
    fn msaa_enable(&mut self) -> &mut bool;
    fn ssaa_enable(&mut self) -> &mut bool;
    fn create_frame_buffer(&self, size: UVec2) -> Box<dyn FrameBuffer>;
    fn frame_size(&self) -> UVec2;
    fn resize_frame(&mut self, new_size: UVec2);
    fn render_current_frame_if_ready(&self, f: &dyn Fn(&mut dyn FrameBuffer));
    fn last_frame_time(&self) -> std::time::Duration;
    fn present(&self, ctx: &egui::Context) -> egui::TextureId;
}
