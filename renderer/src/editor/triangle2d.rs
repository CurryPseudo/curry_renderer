use crate::*;

#[derive(Default)]
pub struct Triangle2dWithColorEditor {
    pub vec2_editor_meta: Vec2EditorMeta,
}
impl Triangle2dWithColorEditor {
    pub fn update(
        &self,
        ui: &mut egui::Ui,
        triangle: &mut Triangle2d,
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
