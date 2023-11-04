use crate::*;

pub struct MinimapEditor {
    size: f32,
}
impl MinimapEditor {
    pub fn new(size: f32) -> Self {
        Self { size }
    }
}
impl MinimapEditor {
    pub fn update(
        &self,
        ui: &mut egui::Ui,
        viewport_editor: &ViewportEditor,
        viewport_texture_id: egui::TextureId,
    ) {
        if viewport_editor.zoom_in <= 1.0 {
            return;
        }
        let max_rect = ui.max_rect();
        let rect =
            egui::Rect::from_min_max(max_rect.max - egui::Vec2::splat(self.size), max_rect.max);
        ui.allocate_ui_at_rect(rect, |ui| {
            let (_response, painter) = ui.allocate_painter(rect.size(), egui::Sense::hover());
            let rect =
                egui::Rect::from_center_size(rect.center(), rect.size() - egui::Vec2::splat(4.0));
            painter.rect(
                rect,
                egui::Rounding::default(),
                egui::Color32::BLACK,
                egui::Stroke::new(1.0, egui::Color32::WHITE),
            );
            let rect =
                egui::Rect::from_center_size(rect.center(), rect.size() - egui::Vec2::splat(2.0));
            painter.image(
                viewport_texture_id,
                rect,
                egui::Rect::from_min_max(egui::Pos2::ZERO, egui::Pos2::new(1.0, 1.0)),
                egui::Color32::WHITE,
            );
            let min = rect.min
                + (rect.size().as_vec2()
                    * (viewport_editor.zoom_offset / viewport_editor.world_size))
                    .as_egui_vec2();
            let max = min + rect.size() * (1.0 / viewport_editor.zoom_in);
            let rect = egui::Rect::from_min_max(min, max);
            painter.rect_stroke(
                rect,
                egui::Rounding::default(),
                egui::Stroke::new(1.0, egui::Color32::WHITE),
            );
        });
    }
}
