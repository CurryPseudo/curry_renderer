use crate::*;
pub struct TriangleEditor {
    pub x_range: std::ops::RangeInclusive<f32>,
    pub y_range: std::ops::RangeInclusive<f32>,
}

impl TriangleEditor {
    pub fn update(&self, ui: &mut egui::Ui, triangle: &mut Triangle) {
        ui.add(egui::Slider::new(&mut triangle[0].x, self.x_range.clone()).text("a.x"));
        ui.add(egui::Slider::new(&mut triangle[0].y, self.y_range.clone()).text("a.y"));
        ui.add(egui::Slider::new(&mut triangle[1].x, self.x_range.clone()).text("b.x"));
        ui.add(egui::Slider::new(&mut triangle[1].y, self.y_range.clone()).text("b.y"));
        ui.add(egui::Slider::new(&mut triangle[2].x, self.x_range.clone()).text("c.x"));
        ui.add(egui::Slider::new(&mut triangle[2].y, self.y_range.clone()).text("c.y"));
    }
}

#[derive(Default)]
pub struct TriangleWithColorEditor {
    pub vec2_editor_meta: Vec2EditorMeta,
}
impl TriangleWithColorEditor {
    pub fn update(
        &self,
        ui: &mut egui::Ui,
        triangle: &mut Triangle,
        triangle_colors: &mut [egui::Color32; 3],
    ) {
        for i in 0..3 {
            ui.horizontal(|ui| {
                ui.label(format!("P{i}: "));
                Vec2Editor::new(self.vec2_editor_meta.clone()).update(ui, &mut triangle[i]);
                ui.color_edit_button_srgba(&mut triangle_colors[i]);
            });
        }
    }
}
