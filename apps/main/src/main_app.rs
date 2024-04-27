use curry_renderer::*;
use crate::*;

#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)]
pub struct AppPersistentState {}


pub struct MainApp {
    persistent_state: AppPersistentState,
    app_factories: Vec<AppFactory>,
    current_app: Box<dyn App>,
}

impl MainApp {
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
        let app_factories: Vec<AppFactory> = vec![
            AppFactory {
                name: "Triangle3D",
                factory: Box::new(|| Box::new(Triangle3DApp::default())),
            },
            AppFactory {
                name: "Triangle2D",
                factory: Box::new(|| Box::new(Triangle2DApp::default())),
            },
        ];
        let current_app = app_factories[0].new_app();
        Self {
            persistent_state,
            app_factories,
            current_app,
        }
    }
}

impl eframe::App for MainApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.persistent_state);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("App", |ui| {
                    for app_factory in &self.app_factories {
                        if ui.radio(app_factory.name == self.current_app.name(), app_factory.name).clicked() {
                            self.current_app = app_factory.new_app();
                            ui.close_menu();
                        }
                    }
                });
            });
        });
        self.current_app.update(ctx, frame);
    }
}
