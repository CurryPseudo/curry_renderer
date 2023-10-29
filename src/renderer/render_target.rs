pub trait RenderTarget {
    fn as_egui_image_data_mut(&mut self) -> &mut egui::ImageData;
}

impl RenderTarget for egui::ImageData {
    fn as_egui_image_data_mut(&mut self) -> &mut egui::ImageData {
        self
    }
}
