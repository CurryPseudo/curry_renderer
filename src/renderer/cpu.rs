use crate::*;

#[derive(Default)]
pub struct CpuRenderer {
    antialiasing_config: AntialiasingConfig,
}

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
        let AntialiasingConfig {
            msaa_enable,
            msaa_use_area,
            ssaa_enable: _,
        } = self.antialiasing_config;
        let image_data = target.as_egui_image_data_mut();
        match image_data {
            egui::ImageData::Color(color_image) => {
                let mut new_color_image = color_image.as_ref().clone();
                let size = new_color_image.size;
                for y in 0..size[1] {
                    for x in 0..size[0] {
                        let p = vec2(x as f32, y as f32) + Vec2::splat(0.5);
                        if msaa_enable {
                            if msaa_use_area {
                                let mut area_sum = 0.0;
                                for x in 0..2 {
                                    for y in 0..2 {
                                        let sub_pixel = Rect::from_center_size(
                                            p - Vec2::splat(0.25) + vec2(x as f32, y as f32) * 0.5,
                                            Vec2::splat(0.5),
                                        );
                                        let area = overlap_area(triangle, &sub_pixel);
                                        area_sum += area;
                                    }
                                }
                                if area_sum > 0.0 {
                                    new_color_image.pixels[y * size[0] + x] =
                                        egui::Color32::from_rgb((area_sum * 255.0) as u8, 0, 0);
                                }
                            } else {
                                let mut area_sum = 0.0;
                                for x in 0..2 {
                                    for y in 0..2 {
                                        let sub_p =
                                            p - Vec2::splat(0.25) + vec2(x as f32, y as f32) * 0.5;
                                        if triangle.contains(sub_p) {
                                            area_sum += 0.25;
                                        }
                                    }
                                }
                                if area_sum > 0.0 {
                                    assert!(area_sum <= 1.0);
                                    new_color_image.pixels[y * size[0] + x] =
                                        egui::Color32::from_rgb((area_sum * 255.0) as u8, 0, 0);
                                }
                            }
                        } else {
                            if triangle.contains(p) {
                                new_color_image.pixels[y * size[0] + x] = egui::Color32::RED;
                            }
                        }
                    }
                }
                *color_image = Arc::new(new_color_image);
            }
            egui::ImageData::Font(_) => unreachable!(),
        }
    }

    fn antialiasing_config_mut(&mut self) -> &mut AntialiasingConfig {
        &mut self.antialiasing_config
    }
}
