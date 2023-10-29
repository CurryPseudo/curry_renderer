use std::sync::Arc;

use crate::*;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct AppPersistentState {}
pub struct App {
    persistent_state: AppPersistentState,
    viewport_image_data: egui::ImageData,
    viewport_texture_handle: egui::TextureHandle,
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

        let viewport_image_data: egui::ImageData = egui::ImageData::Color(Arc::new(
            egui::ColorImage::new([1, 1], egui::Color32::BLACK),
        ));

        let viewport_texture_handle = cc.egui_ctx.load_texture(
            "viewport",
            viewport_image_data.clone(),
            egui::TextureOptions {
                magnification: egui::TextureFilter::Nearest,
                minification: egui::TextureFilter::Nearest,
            },
        );
        Self {
            persistent_state,
            viewport_image_data,
            viewport_texture_handle,
            triangle: Triangle::new(vec2(370.0, 320.0), vec2(490.0, 120.0), vec2(200.0, 220.0)),
            renderer: Box::new(CpuRenderer::default()),
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
            let viewport_width = self.viewport_image_data.width();
            let viewport_height = self.viewport_image_data.height();
            ui.heading("Viewport");
            ui.label(format!("Width: {}", viewport_width));
            ui.label(format!("Height: {}", viewport_height));
            ui.heading("Renderer");
            let mut msaa_enable = self.renderer.get_msaa_enable();
            if ui.checkbox(&mut msaa_enable, "MSAA").changed() {
                self.renderer.set_msaa_enable(msaa_enable);
            };
            ui.heading("Triangle");
            let viewport_width = viewport_width as f32;
            let viewport_height = viewport_height as f32;
            ui.add(egui::Slider::new(&mut self.triangle.a.x, 0.0..=viewport_width).text("a.x"));
            ui.add(egui::Slider::new(&mut self.triangle.a.y, 0.0..=viewport_height).text("a.y"));
            ui.add(egui::Slider::new(&mut self.triangle.b.x, 0.0..=viewport_width).text("b.x"));
            ui.add(egui::Slider::new(&mut self.triangle.b.y, 0.0..=viewport_height).text("b.y"));
            ui.add(egui::Slider::new(&mut self.triangle.c.x, 0.0..=viewport_width).text("c.x"));
            ui.add(egui::Slider::new(&mut self.triangle.c.y, 0.0..=viewport_height).text("c.y"));
            ui.heading("Performance");
            ui.label(format!(
                "Frame time: {:.3}ms",
                self.frame_time.as_secs_f64() * 1000.0
            ));
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let expect_ui_size = ui.available_size();
            let expect_image_size = [expect_ui_size.x as usize, expect_ui_size.y as usize];
            if expect_image_size != self.viewport_image_data.size() {
                self.viewport_image_data = egui::ImageData::Color(Arc::new(egui::ColorImage::new(
                    expect_image_size,
                    egui::Color32::BLACK,
                )));
            }
            self.viewport_texture_handle = ctx.load_texture(
                "viewport",
                self.viewport_image_data.clone(),
                egui::TextureOptions {
                    magnification: egui::TextureFilter::Nearest,
                    minification: egui::TextureFilter::Nearest,
                },
            );
            ui.image((self.viewport_texture_handle.id(), expect_ui_size));
        });
        self.renderer.clear(&mut self.viewport_image_data);
        self.renderer
            .draw_triangle(&self.triangle, &mut self.viewport_image_data);
        self.frame_time = frame_begin.elapsed();
    }
}
