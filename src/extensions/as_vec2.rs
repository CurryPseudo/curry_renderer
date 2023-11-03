use crate::*;
pub trait AsVec2 {
    fn as_vec2(&self) -> Vec2;
}

impl AsVec2 for egui::Pos2 {
    fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

impl AsVec2 for egui::Vec2 {
    fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}
