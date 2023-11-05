use crate::*;

#[derive(Default)]
pub struct Triangle3dWithColorEditor {
    pub vec3_editor_meta: Vec3EditorMeta,
}
impl Triangle3dWithColorEditor {
    pub fn update(
        &self,
        ui: &mut egui::Ui,
        triangle: &mut Triangle3d,
        triangle_colors: &mut [egui::Color32; 3],
    ) {
        for i in 0..3 {
            ui.horizontal(|ui| {
                ui.label(format!("P{i}: "));
                Vec3Editor::new(self.vec3_editor_meta.clone()).update(ui, &mut triangle[i]);
                ui.color_edit_button_srgba(&mut triangle_colors[i]);
            });
        }
    }
}
