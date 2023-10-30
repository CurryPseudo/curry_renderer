use crate::*;
pub struct ViewportEditor {
    grabing: bool,
    zoom_in: f32,
    zoom_offset: Vec2,
    world_size: Vec2,
}

impl Default for ViewportEditor {
    fn default() -> Self {
        Self {
            grabing: false,
            zoom_in: 1.0,
            zoom_offset: Vec2::ZERO,
            world_size: Vec2::ONE,
        }
    }
}

impl ViewportEditor {
    pub fn set_world_size(&mut self, world_size: Vec2) {
        self.world_size = world_size;
        self.zoom_offset = self
            .zoom_offset
            .clamp(Vec2::ZERO, self.world_size * (1.0 - 1.0 / self.zoom_in));
    }
    pub fn update_side_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Viewport");
        ui.label(format!("World Width: {}", self.world_size.x));
        ui.label(format!("World Height: {}", self.world_size.y));
        ui.label(format!("Zoom: {:.3}", self.zoom_in));
        ui.label(format!("Zoom Offset X: {:.3}", self.zoom_offset.x));
        ui.label(format!("Zoom Offset Y: {:.3}", self.zoom_offset.y));
    }
    pub fn viewport_uv(&self) -> egui::Rect {
        egui::Rect {
            min: (self.zoom_offset / self.world_size).as_egui_pos(),
            max: (self.zoom_offset / self.world_size + 1.0 / self.zoom_in).as_egui_pos(),
        }
    }
    pub fn update_post_viewport(&mut self, ui: &mut egui::Ui, viewport_rect: egui::Rect) {
        ui.input(|input| {
            if !input.pointer.button_down(egui::PointerButton::Primary) {
                self.grabing = false;
            }
            if let Some(cursor_pos) = input.pointer.hover_pos() {
                if viewport_rect.contains(cursor_pos) {
                    let cursor_pos = cursor_pos - viewport_rect.min;
                    if input.pointer.button_down(egui::PointerButton::Primary) {
                        self.grabing = true;
                    }
                    if input.scroll_delta.y != 0.0 {
                        let cursor_world_pos =
                            self.zoom_offset + cursor_pos.as_vec2() / self.zoom_in;
                        self.zoom_in *= 1.0 + input.scroll_delta.y / 1000.0;
                        self.zoom_in = self.zoom_in.clamp(1.0, 100.0);
                        self.zoom_offset = cursor_world_pos - cursor_pos.as_vec2() / self.zoom_in;
                    }
                }
            }
            if self.grabing {
                let delta = input.pointer.delta().as_vec2();
                self.zoom_offset -= delta / self.zoom_in;
            }
            self.zoom_offset = self
                .zoom_offset
                .clamp(Vec2::ZERO, self.world_size * (1.0 - 1.0 / self.zoom_in));
        });
        if self.grabing {
            ui.output_mut(|output| {
                output.cursor_icon = egui::CursorIcon::Grabbing;
            })
        }
    }
}
