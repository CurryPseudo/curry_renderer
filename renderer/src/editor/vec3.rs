use crate::*;
#[derive(Default, Clone)]
pub struct Vec3EditorMeta {
    pub x_range: Option<std::ops::RangeInclusive<f32>>,
    pub y_range: Option<std::ops::RangeInclusive<f32>>,
    pub z_range: Option<std::ops::RangeInclusive<f32>>,
}

#[derive(Default)]
pub struct Vec3Editor(Vec3EditorMeta);
impl Vec3Editor {
    pub fn new(meta: Vec3EditorMeta) -> Self {
        Self(meta)
    }
}

impl Vec3Editor {
    pub fn update(&self, ui: &mut egui::Ui, vec3: &mut Vec3) {
        let Vec3EditorMeta {
            x_range,
            y_range,
            z_range,
        } = &self.0;
        ui.horizontal(|ui| {
            if let Some(x_range) = &x_range {
                ui.add(egui::Slider::new(&mut vec3.x, x_range.clone()).text("x"));
            } else {
                ui.add(egui::DragValue::new(&mut vec3.x).speed(1.0));
                ui.label("x");
            }
            if let Some(y_range) = &y_range {
                ui.add(egui::Slider::new(&mut vec3.y, y_range.clone()).text("y"));
            } else {
                ui.add(egui::DragValue::new(&mut vec3.y).speed(1.0));
                ui.label("y");
            }
            if let Some(z_range) = &z_range {
                ui.add(egui::Slider::new(&mut vec3.z, z_range.clone()).text("z"));
            } else {
                ui.add(egui::DragValue::new(&mut vec3.z).speed(1.0));
                ui.label("z");
            }
        });
    }
}
