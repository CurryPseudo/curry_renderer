use crate::*;
#[derive(Default, Clone)]
pub struct Vec2EditorMeta {
    pub x_range: Option<std::ops::RangeInclusive<f32>>,
    pub y_range: Option<std::ops::RangeInclusive<f32>>,
}

#[derive(Default)]
pub struct Vec2Editor(Vec2EditorMeta);
impl Vec2Editor {
    pub fn new(meta: Vec2EditorMeta) -> Self {
        Self(meta)
    }
}

impl Vec2Editor {
    pub fn update(&self, ui: &mut egui::Ui, vec2: &mut Vec2) {
        let Vec2EditorMeta { x_range, y_range } = &self.0;
        ui.horizontal(|ui| {
            if let Some(x_range) = &x_range {
                ui.add(egui::Slider::new(&mut vec2.x, x_range.clone()).text("x"));
            } else {
                ui.add(egui::DragValue::new(&mut vec2.x).speed(1.0));
                ui.label("x");
            }
            if let Some(y_range) = &y_range {
                ui.add(egui::Slider::new(&mut vec2.y, y_range.clone()).text("y"));
            } else {
                ui.add(egui::DragValue::new(&mut vec2.y).speed(1.0));
                ui.label("y");
            }
        });
    }
}
