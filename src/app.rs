use std::sync::Arc;

use egui::ColorImage;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct AppPersistentState {}
pub struct App {
    persistent_state: AppPersistentState,
    viewport_image_data: egui::ImageData,
    viewport_texture_handle: egui::TextureHandle,
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

        let viewport_image_data: egui::ImageData =
            egui::ImageData::Color(Arc::new(ColorImage::new([1, 1], egui::Color32::BLACK)));

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
        egui::SidePanel::new(egui::panel::Side::Left, "left_panel").show(ctx, |ui| {
            ui.label(format!(
                "viewport width: {}",
                self.viewport_image_data.width()
            ));
            ui.label(format!(
                "viewport height: {}",
                self.viewport_image_data.height()
            ));
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let expect_ui_size = ui.available_size();
            let expect_image_size = [expect_ui_size.x as usize, expect_ui_size.y as usize];
            if expect_image_size != self.viewport_image_data.size() {
                self.viewport_image_data = egui::ImageData::Color(Arc::new(ColorImage::new(
                    expect_image_size,
                    egui::Color32::BLACK,
                )));
                self.viewport_texture_handle = ctx.load_texture(
                    "viewport",
                    self.viewport_image_data.clone(),
                    egui::TextureOptions {
                        magnification: egui::TextureFilter::Nearest,
                        minification: egui::TextureFilter::Nearest,
                    },
                );
            }
            ui.image((self.viewport_texture_handle.id(), expect_ui_size));
        });
    }
}
