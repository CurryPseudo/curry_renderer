use crate::*;

#[derive(Default)]
pub struct CpuRenderer {}

impl Renderer for CpuRenderer {
    fn clear(&self, target: &mut dyn RenderTarget) {
        let image_data = target.as_egui_image_data_mut();
        match image_data {
            egui::ImageData::Color(color_image) => {
                let mut new_color_image = color_image.as_ref().clone();
                new_color_image.pixels.fill(egui::Color32::BLACK);
                *color_image = Arc::new(new_color_image);
            }
            egui::ImageData::Font(_) => unreachable!(),
        }
    }

    fn draw_triangle(&self, triangle: &Triangle, target: &mut dyn RenderTarget) {
        let image_data = target.as_egui_image_data_mut();
        match image_data {
            egui::ImageData::Color(color_image) => {
                let mut new_color_image = color_image.as_ref().clone();
                let size = new_color_image.size;
                for y in 0..size[1] {
                    for x in 0..size[0] {
                        let p = vec2(x as f32, y as f32);
                        if triangle.contains(p) {
                            new_color_image.pixels[y * size[0] + x] = egui::Color32::RED;
                        }
                    }
                }
                *color_image = Arc::new(new_color_image);
            }
            egui::ImageData::Font(_) => unreachable!(),
        }
    }
}
