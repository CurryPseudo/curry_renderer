use crate::*;

mod render_target;
pub use render_target::*;
mod cpu;
pub use cpu::*;
pub trait Renderer {
    fn clear(&self, target: &mut dyn RenderTarget);
    fn draw_triangle(&self, triangle: &Triangle, target: &mut dyn RenderTarget);
}
