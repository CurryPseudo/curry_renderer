use crate::*;
pub trait AsUVec2 {
    fn as_uvec2(&self) -> UVec2;
}

impl AsUVec2 for egui::Vec2 {
    fn as_uvec2(&self) -> UVec2 {
        UVec2::new(self.x as u32, self.y as u32)
    }
}

impl AsUVec2 for [usize; 2] {
    fn as_uvec2(&self) -> UVec2 {
        UVec2::new(self[0] as u32, self[1] as u32)
    }
}
