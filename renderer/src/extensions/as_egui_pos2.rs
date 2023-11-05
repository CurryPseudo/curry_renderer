use crate::*;
pub trait AsEguiPos {
    fn as_egui_pos(&self) -> egui::Pos2;
}

impl AsEguiPos for Vec2 {
    fn as_egui_pos(&self) -> egui::Pos2 {
        egui::Pos2::new(self.x, self.y)
    }
}
