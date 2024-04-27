pub mod triangle3d;

pub use triangle3d::*;

pub mod triangle2d;

pub use triangle2d::*;

pub struct AppFactory {
    pub name: &'static str,
    pub factory: Box<dyn Fn() -> Box<dyn App>>,
}

impl AppFactory {
    pub fn new_app(&self) -> Box<dyn App> {
        (self.factory)()
    }
}

pub trait App {
    fn name(&self) -> &str;
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame);
}
