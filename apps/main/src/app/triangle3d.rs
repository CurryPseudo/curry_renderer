use eframe::Frame;
use egui::Context;
use crate::*;

pub struct Triangle3DApp {
    triangle_0: Triangle3d,
    triangle_0_colors: [egui::Color32; 3],
    triangle_1: Triangle3d,
    triangle_1_colors: [egui::Color32; 3],
    renderer: Box<dyn Renderer>,
    viewport_editor: ViewportEditor,
    minimap_editor: MinimapEditor,
    frame_time: Duration,
}

impl Default for Triangle3DApp {
    fn default() -> Self {
        let renderer = Box::<CpuRenderer>::default();
        Self {
            triangle_0: Triangle3d::new(
                vec3(370.0, 320.0, 0.0),
                vec3(490.0, 120.0, 0.0),
                vec3(200.0, 220.0, 0.0),
            ),
            triangle_0_colors: [
                egui::Color32::RED,
                egui::Color32::BLUE,
                egui::Color32::YELLOW,
            ],
            triangle_1: Triangle3d::new(
                vec3(320.0, 370.0, 0.0),
                vec3(120.0, 490.0, 0.0),
                vec3(220.0, 200.0, 0.0),
            ),
            triangle_1_colors: [
                egui::Color32::BLUE,
                egui::Color32::GOLD,
                egui::Color32::GREEN,
            ],

            renderer,
            frame_time: Default::default(),
            viewport_editor: ViewportEditor::default(),
            minimap_editor: MinimapEditor::new(200.0),
        }
    }
}

impl App for Triangle3DApp {
    fn name(&self) -> &str {
        "Triangle3D"
    }

    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let frame_size = self.renderer.frame_size();
        egui::SidePanel::new(egui::panel::Side::Left, "left_panel").show(ctx, |ui| {
            egui::ScrollArea::new([false, true]).show(ui, |ui| {
                self.viewport_editor.update_side_panel(ui);
                ui.heading("Renderer");
                ui.checkbox(self.renderer.msaa_enable(), "MSAA");
                ui.checkbox(self.renderer.ssaa_enable(), "SSAA");
                ui.heading("Triangle 0");
                //let frame_size = frame_size.as_vec2();
                Triangle3dWithColorEditor::default().update(
                    ui,
                    &mut self.triangle_0,
                    &mut self.triangle_0_colors,
                );
                ui.heading("Triangle 1");
                Triangle3dWithColorEditor::default().update(
                    ui,
                    &mut self.triangle_1,
                    &mut self.triangle_1_colors,
                );
                ui.heading("Performance");
                ui.label(format!(
                    "Frame time: {:.3}ms",
                    self.frame_time.as_secs_f64() * 1000.0
                ));
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let expect_ui_size = ui.available_size();
            let expect_image_size = expect_ui_size.as_uvec2();
            if expect_image_size != frame_size {
                self.renderer.resize_frame(expect_image_size);
                self.viewport_editor
                    .set_world_size(expect_image_size.as_vec2());
            }
            let triangle_0 = self.triangle_0;
            let triangle_0_colors = self.triangle_0_colors;
            let triangle_1 = self.triangle_1;
            let triangle_1_colors = self.triangle_1_colors;
            self.renderer
                .render_current_frame_if_ready(Box::new(move |cmd_list, fb| {
                    let mut rt = cmd_list.create_render_target(fb.size());
                    cmd_list.clear(rt.as_mut());
                    cmd_list.draw_triangle3d(&triangle_0, &triangle_0_colors, rt.as_mut());
                    cmd_list.draw_triangle3d(&triangle_1, &triangle_1_colors, rt.as_mut());
                    cmd_list.copy_render_target_to_frame_buffer(rt.as_ref(), fb);
                }));
            self.frame_time = self.renderer.last_frame_time();
            let viewport_texture_id = self.renderer.present(ctx);
            let viewport_rect = ui
                .add(
                    egui::Image::new((viewport_texture_id, expect_ui_size))
                        .uv(self.viewport_editor.viewport_uv()),
                )
                .rect;
            self.viewport_editor.update_post_viewport(ui, viewport_rect);
            self.minimap_editor
                .update(ui, &self.viewport_editor, viewport_texture_id);
        });
        ctx.request_repaint();
    }
}