use crate::*;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct AppPersistentState {}
pub struct App {
    persistent_state: AppPersistentState,
    frame_buffer: Box<dyn FrameBuffer>,
    triangle: Triangle,
    renderer: Box<dyn Renderer>,
    frame_time: std::time::Duration,
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
            frame_buffer: renderer.create_frame_buffer(UVec2::ONE),
            triangle: Triangle::new(vec2(370.0, 320.0), vec2(490.0, 120.0), vec2(200.0, 220.0)),
            renderer,
            frame_time: Default::default(),
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
        let frame_begin = std::time::Instant::now();
        egui::SidePanel::new(egui::panel::Side::Left, "left_panel").show(ctx, |ui| {
            let size = self.frame_buffer.size();
            ui.heading("Viewport");
            ui.label(format!("Width: {}", size.x));
            ui.label(format!("Height: {}", size.y));
            ui.heading("Renderer");
            let antialiasing_config = self.renderer.antialiasing_config_mut();
            ui.checkbox(&mut antialiasing_config.msaa_enable, "MSAA");
            if ui
                .checkbox(&mut antialiasing_config.ssaa_enable, "SSAA")
                .changed()
            {
                self.frame_buffer = self.renderer.create_frame_buffer(self.frame_buffer.size());
            };
            let size = size.as_vec2();
            ui.heading("Triangle");
            ui.add(egui::Slider::new(&mut self.triangle.a.x, 0.0..=size.x).text("a.x"));
            ui.add(egui::Slider::new(&mut self.triangle.a.y, 0.0..=size.y).text("a.y"));
            ui.add(egui::Slider::new(&mut self.triangle.b.x, 0.0..=size.x).text("b.x"));
            ui.add(egui::Slider::new(&mut self.triangle.b.y, 0.0..=size.y).text("b.y"));
            ui.add(egui::Slider::new(&mut self.triangle.c.x, 0.0..=size.x).text("c.x"));
            ui.add(egui::Slider::new(&mut self.triangle.c.y, 0.0..=size.y).text("c.y"));
            ui.heading("Performance");
            ui.label(format!(
                "Frame time: {:.3}ms",
                self.frame_time.as_secs_f64() * 1000.0
            ));
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let expect_ui_size = ui.available_size();
            let expect_image_size = expect_ui_size.as_uvec2();
            if expect_image_size != self.frame_buffer.size() {
                self.frame_buffer.resize(expect_image_size);
            }
            self.renderer
                .clear(self.frame_buffer.as_render_target_mut());
            self.renderer
                .draw_triangle(&self.triangle, self.frame_buffer.as_render_target_mut());
            self.frame_time = frame_begin.elapsed();
            ui.image((self.frame_buffer.as_egui_texture_id(ctx), expect_ui_size));
        });
    }
}
