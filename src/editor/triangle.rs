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

#[derive(Default)]
pub struct TriangleWithColorEditor {
    pub x_range: Option<std::ops::RangeInclusive<f32>>,
    pub y_range: Option<std::ops::RangeInclusive<f32>>,
}
impl TriangleWithColorEditor {
    pub fn update(
        &self,
        ui: &mut egui::Ui,
        triangle: &mut Triangle,
        triangle_colors: &mut [egui::Color32; 3],
    ) {
        macro_rules! triangle_x {
            ($value: expr, $label: expr) => {
                if let Some(x_range) = &self.x_range {
                    ui.add(egui::Slider::new(&mut $value, x_range.clone()).text($label));
                } else {
                    ui.add(egui::DragValue::new(&mut $value).speed(1.0));
                }
            };
        }
        macro_rules! triangle_y {
            ($value: expr, $label: expr) => {
                if let Some(y_range) = &self.y_range {
                    ui.add(egui::Slider::new(&mut $value, y_range.clone()).text($label));
                } else {
                    ui.add(egui::DragValue::new(&mut $value).speed(1.0));
                }
            };
        }
        triangle_x!(triangle.a.x, "a.x");
        triangle_y!(triangle.a.y, "a.y");
        ui.color_edit_button_srgba(&mut triangle_colors[0]);
        triangle_x!(triangle.b.x, "b.x");
        triangle_y!(triangle.b.y, "b.y");
        ui.color_edit_button_srgba(&mut triangle_colors[1]);
        triangle_x!(triangle.c.x, "c.x");
        triangle_y!(triangle.c.y, "c.y");
        ui.color_edit_button_srgba(&mut triangle_colors[2]);
    }
}
