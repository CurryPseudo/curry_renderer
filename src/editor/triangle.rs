use crate::*;
pub struct TriangleEditor {
    pub x_range: std::ops::RangeInclusive<f32>,
    pub y_range: std::ops::RangeInclusive<f32>,
}

impl TriangleEditor {
    pub fn update(&self, ui: &mut egui::Ui, triangle: &mut Triangle) {
        ui.add(egui::Slider::new(&mut triangle.a.x, self.x_range.clone()).text("a.x"));
        ui.add(egui::Slider::new(&mut triangle.a.y, self.y_range.clone()).text("a.y"));
        ui.add(egui::Slider::new(&mut triangle.b.x, self.x_range.clone()).text("b.x"));
        ui.add(egui::Slider::new(&mut triangle.b.y, self.y_range.clone()).text("b.y"));
        ui.add(egui::Slider::new(&mut triangle.c.x, self.x_range.clone()).text("c.x"));
        ui.add(egui::Slider::new(&mut triangle.c.y, self.y_range.clone()).text("c.y"));
    }
}

pub struct TriangleWithColorEditor {
    pub x_range: std::ops::RangeInclusive<f32>,
    pub y_range: std::ops::RangeInclusive<f32>,
}
impl TriangleWithColorEditor {
    pub fn update(
        &self,
        ui: &mut egui::Ui,
        triangle: &mut Triangle,
        triangle_colors: &mut [egui::Color32; 3],
    ) {
        ui.add(egui::Slider::new(&mut triangle.a.x, self.x_range.clone()).text("a.x"));
        ui.add(egui::Slider::new(&mut triangle.a.y, self.y_range.clone()).text("a.y"));
        ui.color_edit_button_srgba(&mut triangle_colors[0]);
        ui.add(egui::Slider::new(&mut triangle.b.x, self.x_range.clone()).text("b.x"));
        ui.add(egui::Slider::new(&mut triangle.b.y, self.y_range.clone()).text("b.y"));
        ui.color_edit_button_srgba(&mut triangle_colors[1]);
        ui.add(egui::Slider::new(&mut triangle.c.x, self.x_range.clone()).text("c.x"));
        ui.add(egui::Slider::new(&mut triangle.c.y, self.y_range.clone()).text("c.y"));
        ui.color_edit_button_srgba(&mut triangle_colors[2]);
    }
}