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
    fn antialiasing_config_mut(&mut self) -> &mut AntialiasingConfig;
    fn create_frame_buffer(&self, size: UVec2) -> Box<dyn FrameBuffer>;
}
