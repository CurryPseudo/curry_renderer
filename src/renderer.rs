use crate::*;

mod render_target;
pub use render_target::*;
mod cpu;
pub use cpu::*;
pub trait Renderer {
    fn clear(&self, target: &mut dyn RenderTarget);
    fn draw_triangle(&self, triangle: &Triangle, target: &mut dyn RenderTarget);
    fn get_msaa_enable(&self) -> bool;
    fn set_msaa_enable(&mut self, enable: bool);
}
