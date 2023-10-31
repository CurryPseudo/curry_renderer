use crate::*;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct AppPersistentState {}
pub struct App {
    persistent_state: AppPersistentState,
    red_triangle: Triangle,
    blue_triangle: Triangle,
    renderer: Box<dyn Renderer>,
    frame_time: std::time::Duration,
    viewport_editor: ViewportEditor,
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let persistent_state = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };
        let renderer = Box::new(CpuRenderer::default());
        Self {
            persistent_state,
            red_triangle: Triangle::new(vec2(370.0, 320.0), vec2(490.0, 120.0), vec2(200.0, 220.0)),
            blue_triangle: Triangle::new(
                vec2(320.0, 370.0),
                vec2(120.0, 490.0),
                vec2(220.0, 200.0),
            ),
            renderer,
            frame_time: Default::default(),
            viewport_editor: ViewportEditor::default(),
        }
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.persistent_state);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame_size = self.renderer.frame_size();
        egui::SidePanel::new(egui::panel::Side::Left, "left_panel").show(ctx, |ui| {
            self.viewport_editor.update_side_panel(ui);
            ui.heading("Renderer");
            ui.checkbox(&mut self.renderer.msaa_enable(), "MSAA");
            ui.checkbox(&mut self.renderer.ssaa_enable(), "SSAA");
            ui.heading("Red Triangle");
            let frame_size = frame_size.as_vec2();
            TriangleEditor {
                x_range: 0.0..=frame_size.x,
                y_range: 0.0..=frame_size.y,
            }
            .update(ui, &mut self.red_triangle);
            ui.heading("Blue Triangle");
            TriangleEditor {
                x_range: 0.0..=frame_size.x,
                y_range: 0.0..=frame_size.y,
            }
            .update(ui, &mut self.blue_triangle);
            ui.heading("Performance");
            ui.label(format!(
                "Frame time: {:.3}ms",
                self.frame_time.as_secs_f64() * 1000.0
            ));
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let expect_ui_size = ui.available_size();
            let expect_image_size = expect_ui_size.as_uvec2();
            if expect_image_size != frame_size {
                self.renderer.resize_frame(expect_image_size);
                self.viewport_editor
                    .set_world_size(expect_image_size.as_vec2());
            }
            let red_triangle = self.red_triangle.clone();
            let blue_triangle = self.blue_triangle.clone();
            self.renderer
                .render_current_frame_if_ready(Box::new(move |cmd_list, fb| {
                    let mut rt = cmd_list.create_render_target(fb.size());
                    cmd_list.clear(rt.as_mut());
                    cmd_list.draw_triangle(&red_triangle, egui::Color32::RED, rt.as_mut());
                    cmd_list.draw_triangle(&blue_triangle, egui::Color32::BLUE, rt.as_mut());
                    cmd_list.copy_render_target_to_frame_buffer(rt.as_ref(), fb);
                }));
            self.frame_time = self.renderer.last_frame_time();
            let viewport_rect = ui
                .add(
                    egui::Image::new((self.renderer.present(ctx), expect_ui_size))
                        .uv(self.viewport_editor.viewport_uv()),
                )
                .rect;
            self.viewport_editor.update_post_viewport(ui, viewport_rect);
        });
        ctx.request_repaint();
    }
}
