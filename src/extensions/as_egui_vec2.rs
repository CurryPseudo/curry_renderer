use crate::*;
pub trait AsEguiVec2 {
    fn as_egui_vec2(&self) -> egui::Vec2;
}

impl AsEguiVec2 for Vec2 {
    fn as_egui_vec2(&self) -> egui::Vec2 {
        egui::Vec2::new(self.x as f32, self.y as f32)
    }
}

impl AsEguiVec2 for UVec2 {
    fn as_egui_vec2(&self) -> egui::Vec2 {
        self.as_vec2().as_egui_vec2()
    }
}
