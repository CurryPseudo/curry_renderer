use crate::*;
pub trait AsEguiColor32 {
    fn as_egui_color32(&self) -> egui::Color32;
}

impl AsEguiColor32 for Vec3 {
    fn as_egui_color32(&self) -> egui::Color32 {
        egui::Color32::from_rgb(
            (self.x * 255.0) as u8,
            (self.y * 255.0) as u8,
            (self.z * 255.0) as u8,
        )
    }
}
